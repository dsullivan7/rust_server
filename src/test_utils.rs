use super::AppState;
use crate::authentication;
use crate::plaid;
use sea_orm::{DatabaseBackend, MockDatabase};

pub struct TestState {
    pub conn: sea_orm::DatabaseConnection,
    pub plaid_client: Box<dyn plaid::IPlaidClient>,
}

impl Default for TestState {
    fn default() -> TestState {
        TestState {
            conn: MockDatabase::new(DatabaseBackend::Postgres).into_connection(),
            plaid_client: Box::new(plaid::MockIPlaidClient::new()),
        }
    }
}

impl TestState {
    pub fn into_app_state(self) -> AppState {
        AppState {
            conn: self.conn,
            plaid_client: self.plaid_client,
        }
    }
}
