use std::sync::Arc;

#[cfg(test)]
use super::AppState;
use crate::authentication;

use sea_orm::DatabaseBackend;

use mockall::predicate::*;

const DEFAULT_AUTH0_ID: &str = "default_auth0_id";
const DEFAULT_AUTH0_TOKEN: &str = "default_auth0_token";

pub struct TestState {
    pub conn: sea_orm::DatabaseConnection,
    pub authentication: Arc<dyn authentication::IAuthentication>,
}

impl Default for TestState {
    fn default() -> TestState {
        TestState {
            conn: MockDatabase::new(DatabaseBackend::Postgres).into_connection(),
            authentication: Box::new(authentication::MockIAuthentication::new()),
        }
    }
}

impl TestState {
    pub fn get_default_auth_header() -> (String, String) {
        (
            "Authorization".to_string(),
            format!("Bearer {}", DEFAULT_AUTH0_TOKEN),
        )
    }

    pub fn into_app_state(self) -> AppState {
        AppState {
            conn: self.conn,
            authentication: self.authentication,
        }
    }

    pub fn with_default_auth(mut self) -> Self {
        let mut auth = Box::new(authentication::MockIAuthentication::new());

        auth.expect_validate_token()
            .with(eq(String::from(DEFAULT_AUTH0_TOKEN)))
            .times(1)
            .returning(|_| {
                Ok(authentication::Claims {
                    sub: DEFAULT_AUTH0_ID.to_string(),
                })
            });

        self.authentication = auth;

        self
    }
}
