#[path = "linked_in_test.rs"]
#[cfg(test)]
mod linked_in_test;

use anyhow::anyhow;
use async_trait::async_trait;
use mockall::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinkedInUser {
    pub id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinkedInShareContent {
    pub share_commentary: String,
}

#[derive(Error, Debug)]
pub enum LinkedInError {
    #[error("http request error")]
    HTTPRequest(anyhow::Error),
    #[error("json decode error")]
    JSONDecode(anyhow::Error),
}

#[automock]
#[async_trait]
pub trait ILinkedInClient: Send + Sync {
    async fn get_me(&self, access_token: String) -> Result<LinkedInUser, LinkedInError>;
    async fn share(
        &self,
        access_token: String,
        author: String,
        commentary: String,
    ) -> Result<(), LinkedInError>;
}

pub struct LinkedInClient {
    api_url: String,
}

impl LinkedInClient {
    pub fn new(api_url: String) -> LinkedInClient {
        LinkedInClient { api_url }
    }

    async fn request(
        &self,
        method: reqwest::Method,
        path: String,
        access_token: String,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, LinkedInError> {
        let client = reqwest::Client::new();

        let url = format!("{}{}", self.api_url, path);

        let mut req = client
            .request(method, url)
            .header("LinkedIn-Version", "202208")
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
    async fn get_me(&self, access_token: String) -> Result<LinkedInUser, LinkedInError> {
        let res = self
            .request(
                reqwest::Method::GET,
                "/v2/me".to_owned(),
                access_token,
                None,
            )
            .await?;

        println!("res");
        println!("{}", serde_json::to_string_pretty(&res).unwrap());
        let user: LinkedInUser = serde_json::value::from_value(res).unwrap();

        Ok(user)
    }

    async fn share(
        &self,
        access_token: String,
        author: String,
        commentary: String,
    ) -> Result<(), LinkedInError> {
        let res = self
            .request(
                reqwest::Method::POST,
                "/rest/posts".to_owned(),
                access_token,
                Some(serde_json::json!({
                  "author": format!("urn:li:person:{}", author),
                  "commentary": commentary.to_owned(),
                  "visibility": "CONNECTIONS",
                  "lifecycleState": "PUBLISHED",
                  "distribution": {
                    "feedDistribution": "MAIN",
                    "targetEntities": [],
                    "thirdPartyDistributionChannels": []
                  }
                })),
            )
            .await?;

        println!("{}", serde_json::to_string_pretty(&res).unwrap());
        Ok(())
    }
}
