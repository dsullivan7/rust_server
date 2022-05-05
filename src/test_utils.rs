use super::AppState;
use crate::plaid;
use sea_orm::{DatabaseBackend, MockDatabase};

pub struct TestState {
    pub conn: sea_orm::DatabaseConnection,
    pub plaid_client: plaid::MockIPlaidClient,
}

impl TestState {
    pub fn new() -> TestState {
        TestState {
            conn: MockDatabase::new(DatabaseBackend::Postgres).into_connection(),
            plaid_client: plaid::MockIPlaidClient::new(),
        }
    }

    pub fn into_app_state(self) -> AppState {
        AppState {
            conn: self.conn,
            plaid_client: Box::new(self.plaid_client),
        }
    }
}
