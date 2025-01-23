use std::future;

use mockall::predicate::*;

use crate::authentication::{self, IAuthentication};

const DEFAULT_AUTH0_ID: &str = "default_auth0_id";
const DEFAULT_AUTH0_TOKEN: &str = "default_auth0_token";

pub fn get_default_auth_header() -> (String, String) {
    (
        "Authorization".to_string(),
        format!("Bearer {}", DEFAULT_AUTH0_TOKEN),
    )
}

pub fn get_default_auth() -> Box<dyn IAuthentication> {
    let mut auth = authentication::MockIAuthentication::new();

    auth.expect_validate_token()
        .with(eq(String::from(DEFAULT_AUTH0_TOKEN)))
        .times(1)
        .returning(|_| {
            Box::pin(future::ready(Ok(authentication::Claims {
                sub: DEFAULT_AUTH0_ID.to_string(),
            })))
        });

    Box::new(auth)
}
