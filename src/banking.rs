#[path = "banking_test.rs"]
#[cfg(test)]
mod banking_test;

use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use mockall::*;
use reqwest::header::LOCATION;
use reqwest::StatusCode;
use std::collections::HashMap;
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct BankAccount {
    pub dwolla_funding_source_id: Option<String>,
}

#[derive(Clone)]
pub struct BankTransfer {
    pub dwolla_transfer_id: Option<String>,
}

#[derive(Clone)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub dwolla_customer_id: Option<String>,
}

#[derive(Error, Debug)]
pub enum BankingError {
    #[error("http request error")]
    HTTPRequest(anyhow::Error),
    #[error("json decode error")]
    JSONDecode(anyhow::Error),
    #[error("field not found error")]
    FieldNotFound,
    #[error("acccess token not set")]
    AccessTokenNotSet,
}

#[automock]
#[async_trait]
pub trait BankingClient: Send + Sync {
    fn get_plaid_accessor(&self) -> Option<String>;
    async fn create_customer(&self, user: User) -> Result<User, BankingError>;
    async fn create_bank_account(
        &self,
        user: User,
        name: String,
        plaid_processor_token: String,
    ) -> Result<BankAccount, BankingError>;
    async fn create_transfer(
        &self,
        source: BankAccount,
        destination: BankAccount,
        amount: i32,
    ) -> Result<BankTransfer, BankingError>;
}

pub struct DwollaClient {
    api_key: String,
    api_secret: String,
    api_url: String,
    api_access_token: RwLock<Option<String>>,
    api_access_token_expires_at: RwLock<Option<DateTime<Utc>>>,
}

impl DwollaClient {
    pub fn new(api_key: String, api_secret: String, api_url: String) -> DwollaClient {
        DwollaClient {
            api_key,
            api_secret,
            api_url,
            api_access_token: RwLock::new(None),
            api_access_token_expires_at: RwLock::new(None),
        }
    }

    async fn request(
        &self,
        method: reqwest::Method,
        path: String,
        body: Option<serde_json::Value>,
    ) -> Result<String, BankingError> {
        self.authenticate().await?;

        let api_access_token = self
            .api_access_token
            .read()
            .await
            .as_ref()
            .ok_or(BankingError::AccessTokenNotSet)?
            .clone();

        let client = reqwest::Client::new();
        let url = format!("{}{}", self.api_url, path);
        let mut req = client
            .request(method, url)
            .header("Authorization", format!("Bearer {}", api_access_token))
            .header("Accept", "application/vnd.dwolla.v1.hal+json");

        if let Some(body) = &body {
            req = req.json(body);
        }

        let res = req
            .send()
            .await
            .map_err(|err| BankingError::HTTPRequest(anyhow!(err)))?;

        if res.status() == StatusCode::CREATED {
            if let Some(location) = res.headers().get(LOCATION) {
                return Ok(location.to_str().expect("location expected").to_owned());
            }
        }

        log::error!("Unexpected response from dwolla: {}", res.status());

        let text = res
            .text()
            .await
            .map_err(|err| BankingError::HTTPRequest(anyhow!(err)))?;

        Err(BankingError::HTTPRequest(anyhow!(text)))
    }

    async fn authenticate(&self) -> Result<(), BankingError> {
        let mut api_access_token = self.api_access_token.write().await;
        let mut api_access_token_expires_at = self.api_access_token_expires_at.write().await;

        if let Some(expires_at) = *api_access_token_expires_at {
            if expires_at > Utc::now() {
                return Ok(());
            }
        }

        let mut params = HashMap::new();
        params.insert("grant_type", "client_credentials");

        let client = reqwest::Client::new();
        let url = format!("{}{}", self.api_url, "/token");
        let res: serde_json::value::Value = client
            .request(reqwest::Method::POST, url)
            .basic_auth(self.api_key.to_owned(), Some(self.api_secret.to_owned()))
            .form(&params)
            .send()
            .await
            .map_err(|err| BankingError::HTTPRequest(anyhow!(err)))?
            .json()
            .await
            .map_err(|err| BankingError::JSONDecode(anyhow!(err)))?;

        let access_token = res["access_token"]
            .as_str()
            .ok_or(BankingError::FieldNotFound)?
            .to_owned();

        let expires_in = res["expires_in"]
            .as_i64()
            .ok_or(BankingError::FieldNotFound)?
            .to_owned();

        let expires_at = Utc::now() + Duration::seconds(expires_in);

        *api_access_token = Some(access_token);
        *api_access_token_expires_at = Some(expires_at);

        Ok(())
    }
}

#[async_trait]
impl BankingClient for DwollaClient {
    fn get_plaid_accessor(&self) -> Option<String> {
        Some("dwolla".to_owned())
    }

    async fn create_customer(&self, user: User) -> Result<User, BankingError> {
        let res = self
            .request(
                reqwest::Method::POST,
                "/customers".to_owned(),
                Some(serde_json::json!({
                    "firstName":   user.first_name,
                    "lastName":    user.last_name,
                    "email":       format!("dbsullivan+{}@gmail.com", uuid::Uuid::new_v4()),
                    "type":        "personal",
                    "address1":    "address1",
                    "city":        "Brooklyn",
                    "state":       "NY",
                    "postalCode":  "11222",
                    "dateOfBirth": "1980-01-01",
                    "ssn":         "666-55-4321",
                })),
            )
            .await?;

        let dwolla_customer_id = res
            .split('/')
            .collect::<Vec<&str>>()
            .last()
            .ok_or(BankingError::FieldNotFound)?
            .to_string();

        let mut user = user.clone();
        user.dwolla_customer_id = Some(dwolla_customer_id);

        Ok(user)
    }

    async fn create_bank_account(
        &self,
        user: User,
        name: String,
        plaid_processor_token: String,
    ) -> Result<BankAccount, BankingError> {
        let res = self
            .request(
                reqwest::Method::POST,
                format!(
                    "/customers/{}/funding-sources",
                    user.dwolla_customer_id.ok_or(BankingError::FieldNotFound)?
                ),
                Some(serde_json::json!({
                    "name": name,
                    "plaidToken":    plaid_processor_token,
                })),
            )
            .await?;

        let dwolla_funding_source_id = res
            .split('/')
            .collect::<Vec<&str>>()
            .last()
            .ok_or(BankingError::FieldNotFound)?
            .to_string();

        let bank_account = BankAccount {
            dwolla_funding_source_id: Some(dwolla_funding_source_id),
        };

        Ok(bank_account)
    }

    async fn create_transfer(
        &self,
        source: BankAccount,
        destination: BankAccount,
        amount: i32,
    ) -> Result<BankTransfer, BankingError> {
        let res = self
            .request(
                reqwest::Method::POST,
                "/transfers".to_owned(),
                Some(serde_json::json!({
                    "_links": {
        			"source": {
        				"href": format!("{}/funding-sources/{}", self.api_url, source.dwolla_funding_source_id.ok_or(BankingError::FieldNotFound)?),
        			},
        			"destination": {
        				"href": format!("{}/funding-sources/{}", self.api_url, destination.dwolla_funding_source_id.ok_or(BankingError::FieldNotFound)?),
        			},
        		},
        		"amount":{
        			"value": amount / 100,
        			"currency": "USD",
        		},
        	})),
            )
            .await?;

        let dwolla_transfer_id = res
            .split('/')
            .collect::<Vec<&str>>()
            .last()
            .ok_or(BankingError::FieldNotFound)?
            .to_string();

        let bank_transfer = BankTransfer {
            dwolla_transfer_id: Some(dwolla_transfer_id),
        };

        Ok(bank_transfer)
    }
}
