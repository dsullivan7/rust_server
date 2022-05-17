#[path = "plaid_test.rs"]
#[cfg(test)]
mod plaid_test;

use anyhow::anyhow;
use async_trait::async_trait;
use mockall::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlaidError {
    #[error("http request error")]
    HTTPRequest(anyhow::Error),
    #[error("json decode error")]
    JSONDecode(anyhow::Error),
    #[error("field not found error")]
    FieldNotFound,
}

#[derive(Clone)]
pub struct PlaidAccount {
    pub name: Option<String>,
    pub account_id: Option<String>,
    pub access_token: String,
}

#[automock]
#[async_trait]
pub trait IPlaidClient: Send + Sync {
    async fn create_token(&self, user_id: String) -> Result<String, PlaidError>;
    async fn get_access_token(&self, public_token: String) -> Result<PlaidAccount, PlaidError>;
    async fn get_account(&self, mut account: PlaidAccount) -> Result<PlaidAccount, PlaidError>;
    async fn create_processor_token(
        &self,
        account: PlaidAccount,
        accessor: String,
    ) -> Result<String, PlaidError>;
}

pub struct PlaidClient {
    client_id: String,
    secret: String,
    api_url: String,
    redirect_url: String,
}

impl PlaidClient {
    pub fn new(
        client_id: String,
        secret: String,
        api_url: String,
        redirect_url: String,
    ) -> PlaidClient {
        PlaidClient {
            client_id,
            secret,
            api_url,
            redirect_url,
        }
    }

    async fn request(
        &self,
        method: reqwest::Method,
        path: String,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, PlaidError> {
        let client = reqwest::Client::new();
        let url = format!("{}{}", self.api_url, path);
        let mut req = client
            .request(method, url)
            .header("PLAID-CLIENT-ID", self.client_id.clone())
            .header("PLAID-SECRET", self.secret.clone());

        if body.is_some() {
            req = req.json(&body.unwrap());
        }

        req.send()
            .await
            .map_err(|err| PlaidError::HTTPRequest(anyhow!(err)))?
            .json()
            .await
            .map_err(|err| PlaidError::JSONDecode(anyhow!(err)))
    }
}

#[async_trait]
impl IPlaidClient for PlaidClient {
    async fn create_token(&self, user_id: String) -> Result<String, PlaidError> {
        let res = self
            .request(
                reqwest::Method::POST,
                "/link/token/create".to_string(),
                Some(serde_json::json!({
                    "user": {
                        "client_user_id": user_id,
                    },
                    "client_name":   "Sunburst",
                    "products":      ["auth"],
                    "country_codes": ["US"],
                    "language":      "en",
                    "redirect_uri":  self.redirect_url,
                    "account_filters": {
                        "depository": {
                            "account_subtypes": ["checking"],
                        },
                    },
                })),
            )
            .await?;

        Ok(res["link_token"]
            .as_str()
            .ok_or(PlaidError::FieldNotFound)?
            .to_owned())
    }

    async fn get_access_token(&self, public_token: String) -> Result<PlaidAccount, PlaidError> {
        let res = self
            .request(
                reqwest::Method::POST,
                "/item/public_token/exchange".to_string(),
                Some(serde_json::json!({ "public_token": public_token })),
            )
            .await?;

        let access_token = res["access_token"]
            .as_str()
            .ok_or(PlaidError::FieldNotFound)?
            .to_owned();

        let account = PlaidAccount {
            name: None,
            account_id: None,
            access_token,
        };

        Ok(account)
    }

    async fn get_account(&self, mut account: PlaidAccount) -> Result<PlaidAccount, PlaidError> {
        let account_res = self
            .request(
                reqwest::Method::POST,
                "/accounts/get".to_string(),
                Some(serde_json::json!({ "access_token": account.access_token })),
            )
            .await?;

        let account_id = account_res["accounts"][0]["account_id"]
            .as_str()
            .ok_or(PlaidError::FieldNotFound)?
            .to_owned();

        let institution_id = account_res["item"]["institution_id"]
            .as_str()
            .ok_or(PlaidError::FieldNotFound)?
            .to_owned();

        let institution_res = self
            .request(
                reqwest::Method::POST,
                "/institutions/get_by_id".to_string(),
                Some(serde_json::json!({
                    "institution_id": institution_id,
                    "country_codes": ["US"],
                })),
            )
            .await?;

        let institution_name = institution_res["institution"]["name"]
            .as_str()
            .ok_or(PlaidError::FieldNotFound)?
            .to_owned();

        account.account_id = Some(account_id);
        account.name = Some(institution_name);

        Ok(account)
    }

    async fn create_processor_token(
        &self,
        account: PlaidAccount,
        processor: String,
    ) -> Result<String, PlaidError> {
        let res = self
            .request(
                reqwest::Method::POST,
                "/processor/token/create".to_string(),
                Some(serde_json::json!({
                    "access_token": account.access_token,
                    "account_id":   account.account_id,
                    "processor":    processor,
                })),
            )
            .await?;

        let processor_token = res["processor_token"]
            .as_str()
            .ok_or(PlaidError::FieldNotFound)?
            .to_owned();

        Ok(processor_token)
    }
}
