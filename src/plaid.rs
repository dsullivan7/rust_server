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

pub struct PlaidAccount {
    name: Option<String>,
    account_id: Option<String>,
    access_token: String,
}
pub struct PlaidProcessorToken {
    value: String,
}

#[automock]
#[async_trait]
pub trait IPlaidClient: Send + Sync {
    async fn create_token(&self, user_id: String) -> Result<String, PlaidError>;
    // async fn get_access_token(&self, public_token: String) -> Result<PlaidAccount, PlaidError>;
    // async fn get_account(&self, account: PlaidAccount) -> Result<PlaidAccount, PlaidError>;
    // async fn get_processor_token(
    //     &self,
    //     account: PlaidAccount,
    //     accessor: String,
    // ) -> Result<PlaidProcessorToken, PlaidError>;
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
}
