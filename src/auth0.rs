#[path = "auth0_test.rs"]
#[cfg(test)]
mod auth0_test;

use anyhow::anyhow;
use async_trait::async_trait;
use mockall::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Auth0Identity {
    pub provider: String,
    pub access_token: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Auth0User {
    pub user_id: String,
    pub identities: Vec<Auth0Identity>,
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
    async fn get_user(&self, access_token: String, user_id: String) -> Result<Auth0User, Auth0Error>;
    async fn get_access_token(&self) -> Result<String, Auth0Error>;
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
        access_token: String,
        method: reqwest::Method,
        path: String,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, Auth0Error> {
        let client = reqwest::Client::new();

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
            .map_err(|err| Auth0Error::HTTPRequest(anyhow!(err)))?
            .json()
            .await
            .map_err(|err| Auth0Error::JSONDecode(anyhow!(err)))
    }
}

#[async_trait]
impl IAuth0Client for Auth0Client {
    async fn get_user(&self, access_token: String, auth0_id: String) -> Result<Auth0User, Auth0Error> {
        let res = self
            .request(
                access_token,
                reqwest::Method::GET,
                format!("/api/v2/users/{}", auth0_id),
                None,
            )
            .await?;

        println!("res");
        println!("{}", serde_json::to_string_pretty(&res).unwrap());
        let user: Auth0User = serde_json::value::from_value(res).unwrap();

        Ok(user)
    }

    async fn get_access_token(
      &self,
  ) -> Result<String, Auth0Error> {
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
          .map_err(|err| Auth0Error::HTTPRequest(anyhow!(err)))?
          .json()
          .await
          .map_err(|err| Auth0Error::JSONDecode(anyhow!(err)))?;

      let access_token = access_token_res["access_token"]
          .as_str()
          .ok_or(Auth0Error::FieldNotFound)?
          .to_owned();

      Ok(access_token)
  }
}
