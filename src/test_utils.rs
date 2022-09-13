#[cfg(test)]
use super::AppState;
use crate::authentication;
use crate::banking;
use crate::gov;
use crate::plaid;
use crate::services;

use sea_orm::{DatabaseBackend, MockDatabase};

use mockall::predicate::*;

const DEFAULT_AUTH0_ID: &str = "default_auth0_id";
const DEFAULT_AUTH0_TOKEN: &str = "default_auth0_token";

pub struct TestState {
    pub conn: sea_orm::DatabaseConnection,
    pub plaid_client: Box<dyn plaid::IPlaidClient>,
    pub banking_client: Box<dyn banking::BankingClient>,
    pub gov_client: Box<dyn gov::IGovernment>,
    pub authentication: Box<dyn authentication::IAuthentication>,
    pub services: Box<dyn services::IServices>,
}

impl Default for TestState {
    fn default() -> TestState {
        TestState {
            conn: MockDatabase::new(DatabaseBackend::Postgres).into_connection(),
            plaid_client: Box::new(plaid::MockIPlaidClient::new()),
            banking_client: Box::new(banking::MockBankingClient::new()),
            gov_client: Box::new(gov::MockIGovernment::new()),
            authentication: Box::new(authentication::MockIAuthentication::new()),
            services: Box::new(services::MockIServices::new()),
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
            plaid_client: self.plaid_client,
            banking_client: self.banking_client,
            gov_client: self.gov_client,
            authentication: self.authentication,
            services: self.services,
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
