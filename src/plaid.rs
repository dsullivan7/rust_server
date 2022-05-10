#[path = "plaid_test.rs"]
#[cfg(test)]
mod plaid_test;

use async_trait::async_trait;
use mockall::*;

#[automock]
#[async_trait]
pub trait IPlaidClient: Send + Sync {
    async fn create_token(&self, user_id: String) -> String;
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
    ) -> serde_json::Value {
        let client = reqwest::Client::new();
        let url = format!("{}{}", self.api_url, path);
        let mut req = client
            .request(method, url)
            .header("PLAID-CLIENT-ID", self.client_id.clone())
            .header("PLAID-SECRET", self.secret.clone());

        if body.is_some() {
            req = req.json(&body.unwrap());
        }

        req.send().await.unwrap().json().await.unwrap()
    }
}

#[async_trait]
impl IPlaidClient for PlaidClient {
    async fn create_token(&self, user_id: String) -> String {
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
            .await;

        res["link_token"].as_str().unwrap().to_owned()
    }
}
