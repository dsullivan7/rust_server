use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use mockall::*;
use thiserror::Error;

use crate::models::user::Model as User;
use crate::models::bank_account::Model as BankAccount;
use crate::models::bank_transfer::Model as BankTransfer;

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
    async fn create_customer(&self, user: User) -> Result<User, BankingError>;
    async fn create_bank_account(&self, user: User) -> Result<BankAccount, BankingError>;
    async fn create_bank_transfer(&self, bank_account: BankAccount) -> Result<BankTransfer, BankingError>;
}

pub struct DwollaClient {
    api_key: String,
    api_secret: String,
    api_url: String,
    api_access_token: Option<String>,
    api_access_token_expires_at: Option<DateTime<Utc>>,
}

impl DwollaClient {
    pub fn new(api_key: String, api_secret: String, api_url: String) -> DwollaClient {
        DwollaClient {
            api_key,
            api_secret,
            api_url,
            api_access_token: None,
            api_access_token_expires_at: None,
        }
    }

    async fn request(
        &self,
        method: reqwest::Method,
        path: String,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, BankingError> {
        self.authenticate().await?;

        let api_access_token = self.api_access_token.ok_or(BankingError::AccessTokenNotSet)?;

        let client = reqwest::Client::new();
        let url = format!("{}{}", self.api_url, path);
        let mut req = client
            .request(method, url)
            .header("Authorization", format!("Bearer {}", self.api_access_token));

        if let Some(body) = &body {
            req = req.json(body);
        }

        req.send()
            .await
            .map_err(|err| BankingError::HTTPRequest(anyhow!(err)))?
            .json()
            .await
            .map_err(|err| BankingError::JSONDecode(anyhow!(err)))?
    }

    async fn authenticate(&self) -> Result<(), BankingError> {
        if self.expires_at > Utc::now() {
            return Ok(());
        }

        let client = reqwest::Client::new();
        let url = format!("{}{}", self.api_url, "/token");
        let res = client
            .request(reqwest::Method::POST, url)
            .basic_auth(self.api_key, self.api_secret)
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
            .as_u64()
            .ok_or(BankingError::FieldNotFound)?
            .to_owned();

        let expires_at = Utc::now() + Duration::seconds(expires_in);

        self.api_access_token = Some(access_token);
        self.api_access_token_expires_at = Some(expires_at);

        Ok(())
    }
}

#[async_trait]
impl BankingClient for DwollaClient {
    async fn create_customer(&self, user: User) -> Result<User, BankingError> {
        let res = self
            .request(
                reqwest::Method::POST,
                "/customers".to_owned(),
                Some(serde_json::json!({
                    "firstName":   user.first_name,
                    "lastName":    user.last_name,
                    // "email":       user.Email,
                    // "type":        "personal",
                    // "address1":    user.Address,
                    // "city":        user.City,
                    // "state":       user.State,
                    // "postalCode":  user.PostalCode,
                    // "dateOfBirth": user.DateOfBirth,
                    // "ssn":         user.SSN,
                })),
            )
            .await?;

        let dwolla_customer_id = res["customer_id"]
            .as_str()
            .ok_or(BankingError::FieldNotFound)?
            .to_owned();

        user.dwolla_customer_id = dwolla_customer_id;

        Ok(user)
    }

    async fn create_bank_account(&self, user: User) -> Result<BankAccount, BankingError> {
        let res = self
            .request(
                reqwest::Method::POST,
                "/customers".to_owned(),
                Some(serde_json::json!({
                    "firstName":   user.first_name,
                    "lastName":    user.last_name,
                    // "email":       user.Email,
                    // "type":        "personal",
                    // "address1":    user.Address,
                    // "city":        user.City,
                    // "state":       user.State,
                    // "postalCode":  user.PostalCode,
                    // "dateOfBirth": user.DateOfBirth,
                    // "ssn":         user.SSN,
                })),
            )
            .await?;

        let dwolla_customer_id = res["customer_id"]
            .as_str()
            .ok_or(BankingError::FieldNotFound)?
            .to_owned();

        user.dwolla_customer_id = dwolla_customer_id;

        Ok(user)
    }
}
