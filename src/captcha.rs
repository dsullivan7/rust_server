#[path = "captcha_test.rs"]
#[cfg(test)]
mod captcha_test;

use anyhow::anyhow;
use async_trait::async_trait;
use mockall::*;
use std::{thread, time::Duration};
use thiserror::Error;

const TWO_CAPTCHA_NOT_READY: &str = "CAPCHA_NOT_READY";
const TWO_CAPTCHA_FIRST_WAIT: u64 = 15;
const TWO_CAPTCHA_RETRY_WAIT: u64 = 10;
const MAX_RETRIES: i32 = 10;

#[derive(Error, Debug)]
pub enum CaptchaError {
    #[error("http request error")]
    HTTPRequest(anyhow::Error),
    #[error("json decode error")]
    Decode(anyhow::Error),
    #[error("field not found error")]
    FieldNotFound,
    #[error("client initialization error")]
    ClientInit(anyhow::Error),
    #[error("max retries reached")]
    MaxRetriesReached,
}

#[derive(Clone)]
pub struct TwoCaptcha {
    key: String,
}

#[automock]
#[async_trait]
pub trait Captcha: Send + Sync {
    async fn solve_recaptcha_v2(
        &self,
        google_key: String,
        url: String,
    ) -> Result<String, CaptchaError>;
}

impl TwoCaptcha {
    pub fn new(key: String) -> TwoCaptcha {
        TwoCaptcha { key }
    }

    async fn create_recaptcha_v2_request(
        &self,
        google_key: String,
        url: String,
    ) -> Result<String, CaptchaError> {
        let client = reqwest::Client::new();

        log::info!("making request to solve 2captcha...");
        let complete_req: serde_json::Value = client
            .get(format!(
                "http://2captcha.com/in.php?key={}&method=userrecaptcha&googlekey={}&pageurl={}&json=1",
                self.key, google_key, url
            ))
            .send()
            .await
            .map_err(|err| CaptchaError::HTTPRequest(anyhow!(err)))?
            .json()
            .await
            .map_err(|err| CaptchaError::Decode(anyhow!(err)))?;

        let request_id = complete_req["request"]
            .as_str()
            .ok_or(CaptchaError::FieldNotFound)?
            .to_owned();

        Ok(request_id.to_owned())
    }

    async fn get_recaptcha_v2_response(&self, request_id: String) -> Result<String, CaptchaError> {
        let client = reqwest::Client::builder()
            .pool_max_idle_per_host(0)
            .build()
            .map_err(|err| CaptchaError::ClientInit(anyhow!(err)))?;

        let mut i = 0;

        while i < MAX_RETRIES {
            log::info!("checking status of 2captcha...");
            let complete_req: serde_json::Value = client
                .get(format!(
                    "http://2captcha.com/res.php?key={}&action=get&id={}&json=1",
                    self.key, request_id,
                ))
                .send()
                .await
                .map_err(|err| CaptchaError::HTTPRequest(anyhow!(err)))?
                .json()
                .await
                .map_err(|err| CaptchaError::Decode(anyhow!(err)))?;

            let response = complete_req["request"]
                .as_str()
                .ok_or(CaptchaError::FieldNotFound)?
                .to_owned();

            if response != TWO_CAPTCHA_NOT_READY {
                return Ok(response);
            }

            i = i + 1;

            if i < MAX_RETRIES {
                thread::sleep(Duration::from_secs(TWO_CAPTCHA_RETRY_WAIT));
            }
        }

        Err(CaptchaError::MaxRetriesReached)
    }
}

#[async_trait]
impl Captcha for TwoCaptcha {
    async fn solve_recaptcha_v2(
        &self,
        google_key: String,
        url: String,
    ) -> Result<String, CaptchaError> {
        let captcha_request_id = self.create_recaptcha_v2_request(google_key, url).await?;
        thread::sleep(Duration::from_secs(TWO_CAPTCHA_FIRST_WAIT));
        let captcha_response = self.get_recaptcha_v2_response(captcha_request_id).await?;
        Ok(captcha_response)
    }
}
