#[path = "gov_test.rs"]
#[cfg(test)]
mod gov_test;

use anyhow::anyhow;
use async_trait::async_trait;
use mockall::*;
use scraper::{Html, Selector};
use std::collections::HashMap;
use thiserror::Error;

use crate::captcha;

#[derive(Error, Debug)]
pub enum GovernmentError {
    #[error("http request error")]
    HTTPRequest(anyhow::Error),
    #[error("decode error")]
    Decode(anyhow::Error),
    #[error("field not found error")]
    FieldNotFound,
    #[error("unauthorized error")]
    Unauthorized,
    #[error("client initialization error")]
    ClientInit(anyhow::Error),
    #[error("captcha error")]
    Captcha(anyhow::Error),
    #[error("unrecognized portal type")]
    UnrecognizedPortalType,
    #[error("could not find html element")]
    HTMLDocumentParse(String),
}

pub struct Government {
    captcha: Box<dyn captcha::Captcha>,
}

pub struct Profile {
    ebt_snap_balance: String,
}

#[automock]
#[async_trait]
pub trait IGovernment: Send + Sync {
    async fn get_profile(
        &self,
        username: String,
        password: String,
        ip_address: String,
        portal_type: String,
    ) -> Result<Profile, GovernmentError>;
}

impl Government {
    pub fn new(captcha_client: Box<dyn captcha::Captcha>) -> Government {
        Government {
            captcha: captcha_client,
        }
    }

    async fn get_accesshra_profile(
        &self,
        username: String,
        password: String,
        ip_address: String,
    ) -> Result<Profile, GovernmentError> {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .map_err(|err| GovernmentError::ClientInit(anyhow!(err)))?;

        let url_login = "https://a069-access.nyc.gov/Rest/j_security_check";

        let mut params = HashMap::new();
        params.insert("j_username", username);
        params.insert("j_password", password);
        params.insert("user_type", format!("EXTERNAL;{}", ip_address));

        let res = client
            .request(reqwest::Method::POST, url_login)
            .form(&params)
            .send()
            .await
            .map_err(|err| GovernmentError::HTTPRequest(anyhow!(err)))?;

        if res.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(GovernmentError::Unauthorized);
        }

        let url_payments = "https://a069-access.nyc.gov/Rest/v1/ua/anyc/payments/1";

        let res = client
            .request(reqwest::Method::GET, url_payments)
            .header(
                "Referer",
                "https://a069-access.nyc.gov/accesshra/anycuserhome",
            )
            .send()
            .await
            .map_err(|err| GovernmentError::HTTPRequest(anyhow!(err)))?;

        if res.status() != reqwest::StatusCode::OK {
            let text = res
                .text()
                .await
                .map_err(|err| GovernmentError::Decode(anyhow!(err)))?;
            return Err(GovernmentError::HTTPRequest(anyhow!(text)));
        }

        let res_json: serde_json::Value = res
            .json()
            .await
            .map_err(|err| GovernmentError::Decode(anyhow!(err)))?;

        let ebt_snap_balance = res_json["snapEBTBalance"]
            .as_str()
            .ok_or(GovernmentError::FieldNotFound)?
            .to_owned();

        Ok(Profile { ebt_snap_balance })
    }

    fn get_document_value(
        &self,
        doc: String,
        selector: String,
        attribute: String,
    ) -> Option<String> {
        let document = Html::parse_document(&doc);
        let selector = Selector::parse(&selector).unwrap();
        if let Some(found) = document.select(&selector).next() {
            if let Some(attr) = found.value().attr(&attribute) {
                return Some(attr.to_owned());
            }
        }
        None
    }

    async fn get_connectebt_profile(
        &self,
        username: String,
        password: String,
    ) -> Result<Profile, GovernmentError> {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .map_err(|err| GovernmentError::ClientInit(anyhow!(err)))?;

        let base_url = "https://www.connectebt.com";

        let login_url = format!("{}/nyebtclient/siteLogonClient.recip", base_url);

        let mut params = HashMap::new();
        params.insert("login", username);
        params.insert("password", password);

        let res_login = client
            .request(reqwest::Method::POST, login_url.clone())
            .form(&params)
            .send()
            .await
            .map_err(|err| GovernmentError::HTTPRequest(anyhow!(err)))?
            .text()
            .await
            .map_err(|err| GovernmentError::Decode(anyhow!(err)))?;

        // let res_login = fs::read_to_string("recaptcha_doc.html")
        //     .expect("Something went wrong reading the file");

        // let captcha_path = "blah";
        let captcha_path = self
            .get_document_value(res_login, "#main-iframe".to_owned(), "src".to_owned())
            .ok_or(GovernmentError::HTMLDocumentParse("src".to_owned()))?;

        let res_captcha = client
            .request(
                reqwest::Method::GET,
                format!("{}{}", base_url, captcha_path),
            )
            .send()
            .await
            .map_err(|err| GovernmentError::HTTPRequest(anyhow!(err)))?
            .text()
            .await
            .map_err(|err| GovernmentError::Decode(anyhow!(err)))?;

        // let res_captcha = fs::read_to_string("recaptcha_doc_iframe.html")
        //     .expect("Something went wrong reading the file");

        // let google_sitekey = "blah";
        let google_sitekey = self
            .get_document_value(
                res_captcha,
                ".g-recaptcha".to_owned(),
                "data-sitekey".to_owned(),
            )
            .ok_or(GovernmentError::HTMLDocumentParse(
                "data-sitekey".to_owned(),
            ))?;

        let recaptcha_response = self
            .captcha
            .solve_recaptcha_v2(google_sitekey.to_owned(), login_url.clone())
            .await
            .map_err(|err| GovernmentError::Captcha(anyhow!(err)))?;

        let mut params = HashMap::new();
        params.insert("g-recaptcha-response", recaptcha_response);

        client
            .request(
                reqwest::Method::POST,
                format!("{}/_Incapsula_Resource?SWCGHOEL=v2&cts=NA", base_url),
            )
            .form(&params)
            .send()
            .await
            .map_err(|err| GovernmentError::HTTPRequest(anyhow!(err)))?
            .text()
            .await
            .map_err(|err| GovernmentError::Decode(anyhow!(err)))?;

        let res_after_captcha = client
            .request(reqwest::Method::POST, login_url.clone())
            .form(&params)
            .send()
            .await
            .map_err(|err| GovernmentError::HTTPRequest(anyhow!(err)))?
            .text()
            .await
            .map_err(|err| GovernmentError::Decode(anyhow!(err)))?;

        println!("res_after_captcha");
        println!("{}", res_after_captcha);

        Ok(Profile {
            ebt_snap_balance: "".to_owned(),
        })
    }
}

#[async_trait]
impl IGovernment for Government {
    async fn get_profile(
        &self,
        username: String,
        password: String,
        ip_address: String,
        portal_type: String,
    ) -> Result<Profile, GovernmentError> {
        match portal_type.as_str() {
            "accesshra" => {
                return self
                    .get_accesshra_profile(username, password, ip_address)
                    .await
            }
            "connectebt" => return self.get_connectebt_profile(username, password).await,
            _ => return Err(GovernmentError::UnrecognizedPortalType),
        }
    }
}
