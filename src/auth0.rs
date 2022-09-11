#[path = "auth0_test.rs"]
#[cfg(test)]
mod auth0_test;

use anyhow::anyhow;
use async_trait::async_trait;
use mockall::*;
use serde_json::json;
use thiserror::Error;

#[derive(Clone)]
pub struct Auth0User {
    pub user_id: String,
}

#[derive(Error, Debug)]
pub enum Auth0Error {
    #[error("http request error")]
    HTTPRequest(anyhow::Error),
    #[error("json decode error")]
    JSONDecode(anyhow::Error),
    #[error("field not found error")]
    FieldNotFound,
}

#[automock]
#[async_trait]
pub trait IAuth0Client: Send + Sync {
    async fn get_user(&self, user_id: String) -> Result<Auth0User, Auth0Error>;
}

pub struct Auth0Client {
    client_id: String,
    client_secret: String,
    api_url: String,
}

impl Auth0Client {
    pub fn new(client_id: String, client_secret: String, api_url: String) -> Auth0Client {
        Auth0Client {
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
    ) -> Result<serde_json::Value, Auth0Error> {
        let client = reqwest::Client::new();

        let access_token_req = client
            .request(
                reqwest::Method::GET,
                format!("{}{}", self.api_url, "/oauth/token"),
            )
            .json(&json!({
                "client_id": self.client_id,
                "client_secret": self.client_secret,
                "audience": format!("{}{}", self.api_url, "/api/v2/"),
                "grant_type": "client_credentials",
            }));

        let access_token_res: serde_json::Value = access_token_req
            .send()
            .await
            .map_err(|err| Auth0Error::HTTPRequest(anyhow!(err)))?
            .json()
            .await
            .map_err(|err| Auth0Error::JSONDecode(anyhow!(err)))?;

        let access_token = access_token_res["access_token"]
            .as_str()
            .ok_or(Auth0Error::FieldNotFound)?
            .to_owned();

        let url = format!("{}{}", self.api_url, path);
        let mut req = client
            .request(method, url)
            .header("authorization", access_token);

        if body.is_some() {
            req = req.json(&body.unwrap());
        }

        req.send()
            .await
            .map_err(|err| Auth0Error::HTTPRequest(anyhow!(err)))?
            .json()
            .await
            .map_err(|err| Auth0Error::JSONDecode(anyhow!(err)))
    }
}

#[async_trait]
impl IAuth0Client for Auth0Client {
    async fn get_user(&self, auth0_id: String) -> Result<Auth0User, Auth0Error> {
        let res = self
            .request(reqwest::Method::GET, format!("/users/{}", auth0_id), None)
            .await?;

        let user_id = res["user_id"]
            .as_str()
            .ok_or(Auth0Error::FieldNotFound)?
            .to_owned();

        let user: Auth0User = Auth0User { user_id };

        Ok(user)
    }
}
