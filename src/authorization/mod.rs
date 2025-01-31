use mockall::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum AuthorizationError {
    #[error("authorization error")]
    Error(anyhow::Error),
}

pub struct User {
    pub user_id: Uuid,
    pub role: String,
}

#[automock]
pub trait IAuthorization: Send + Sync {
    fn is_action_allowed(&self, actor: User, action: String) -> Result<bool, AuthorizationError>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Authorization;

impl IAuthorization for Authorization {
    fn is_action_allowed(&self, _actor: User, _action: String) -> Result<bool, AuthorizationError> {
        Ok(true)
    }
}
