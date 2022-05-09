use super::AppState;
use crate::authentication;
use crate::plaid;
use sea_orm::{DatabaseBackend, MockDatabase};

pub struct TestState {
    pub conn: sea_orm::DatabaseConnection,
    pub plaid_client: Box<dyn plaid::IPlaidClient>,
    pub authentication: authentication::Authentication,
}

impl Default for TestState {
    fn default() -> TestState {
        TestState {
            conn: MockDatabase::new(DatabaseBackend::Postgres).into_connection(),
            plaid_client: Box::new(plaid::MockIPlaidClient::new()),
            authentication: authentication::Authentication {
                audience: "audience".to_string(),
                domain: "domain".to_string(),
            },
        }
    }
}

impl TestState {
    pub fn into_app_state(self) -> AppState {
        AppState {
            conn: self.conn,
            plaid_client: self.plaid_client,
            authentication: self.authentication,
        }
    }
}
