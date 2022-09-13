#[path = "linked_in_test.rs"]
#[cfg(test)]
mod linked_in_test;

use anyhow::anyhow;
use async_trait::async_trait;
use mockall::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum LinkedInError {
    #[error("http request error")]
    HTTPRequest(anyhow::Error),
    #[error("json decode error")]
    JSONDecode(anyhow::Error),
    #[error("field not found error")]
    FieldNotFound,
}

#[automock]
#[async_trait]
pub trait ILinkedInClient: Send + Sync {
    async fn get_user(&self, user_id: String) -> Result<LinkedInUser, LinkedInError>;
}

pub struct LinkedInClient {
    client_id: String,
    client_secret: String,
    api_url: String,
}

impl LinkedInClient {
    pub fn new(client_id: String, client_secret: String, api_url: String) -> LinkedInClient {
        LinkedInClient {
            client_id,
            client_secret,
            api_url,
        }
    }

    async fn request(
        &self,
        method: reqwest::Method,
        path: String,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, LinkedInError> {
        let client = reqwest::Client::new();

        let mut access_token_params = HashMap::new();
        access_token_params.insert("client_id", self.client_id.to_owned());
        access_token_params.insert("client_secret", self.client_secret.to_owned());
        access_token_params.insert(
            "audience",
            format!("{}{}", self.api_url, "/api/v2/").to_owned(),
        );
        access_token_params.insert("grant_type", "client_credentials".to_owned());

        let access_token_req = client
            .request(
                reqwest::Method::POST,
                format!("{}{}", self.api_url, "/oauth/token"),
            )
            .form(&access_token_params);

        let access_token_res: serde_json::Value = access_token_req
            .send()
            .await
            .map_err(|err| LinkedInError::HTTPRequest(anyhow!(err)))?
            .json()
            .await
            .map_err(|err| LinkedInError::JSONDecode(anyhow!(err)))?;

        let access_token = access_token_res["access_token"]
            .as_str()
            .ok_or(LinkedInError::FieldNotFound)?
            .to_owned();

        let url = format!("{}{}", self.api_url, path);
        println!("{}", url);
        let mut req = client
            .request(method, url)
            .header("authorization", format!("Bearer {}", access_token));

        if body.is_some() {
            req = req.json(&body.unwrap());
        }

        req.send()
            .await
            .map_err(|err| LinkedInError::HTTPRequest(anyhow!(err)))?
            .json()
            .await
            .map_err(|err| LinkedInError::JSONDecode(anyhow!(err)))
    }
}

#[async_trait]
impl ILinkedInClient for LinkedInClient {
    async fn get_user(&self, linked_in_id: String) -> Result<LinkedInUser, LinkedInError> {
        let res = self
            .request(
                reqwest::Method::GET,
                format!("/api/v2/users/{}", linked_in_id),
                None,
            )
            .await?;

        let user: LinkedInUser = serde_json::value::from_value(res).unwrap();

        Ok(user)
    }
}
