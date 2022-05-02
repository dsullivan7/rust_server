#[path = "plaid_test.rs"]
#[cfg(test)]
mod plaid_test;

use async_trait::async_trait;
use serde_json::{Map, Number, Value};

#[async_trait]
pub trait IPlaidClient {
    async fn create_token(&self) -> String;
}

pub struct PlaidClient;

#[async_trait]
impl IPlaidClient for PlaidClient {
    async fn create_token(&self) -> String {
        let plaid_api_url = std::env::var("PLAID_API_URL").expect("PLAID_API_URL must be set");
        let plaid_client_id =
            std::env::var("PLAID_CLIENT_ID").expect("PLAID_CLIENT_ID must be set");
        let plaid_secret = std::env::var("PLAID_SECRET").expect("PLAID_SECRET must be set");
        let plaid_recirect_uri =
            std::env::var("PLAID_REDIRECT_URI").expect("PLAID_REDIRECT_URI must be set");

        let mut body = Map::new();
        body.insert("client_id", plaid_client_id);
        body.insert("secret", plaid_secret);
        let res = reqwest::get(uri).await.unwrap().json().await;
        res
    }
}
