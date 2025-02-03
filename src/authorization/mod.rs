use mockall::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum AuthorizationError {
    #[error("authorization error")]
    Error,
}

#[derive(Clone)]
pub struct User {
    pub user_id: Uuid,
    pub role: String,
}

#[automock]
pub trait IAuthorization: Send + Sync {
    fn can_get_user(&self, actor: User, resource_id: Uuid) -> Result<bool, AuthorizationError>;
    // fn can_modify_user(&self, actor: User, resource_id: Uuid) -> Result<bool, AuthorizationError>;
    // fn can_delete_user(&self, actor: User, resource_id: Uuid) -> Result<bool, AuthorizationError>;
    fn can_list_users(&self, actor: User) -> Result<bool, AuthorizationError>;
    // fn can_create_user(&self, actor: User) -> Result<bool, AuthorizationError>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Authorization;

impl Authorization {
    fn is_user_admin(&self, actor: User) -> Result<bool, AuthorizationError> {
        if actor.role == "admin" {
            return Ok(true);
        }
        Err(AuthorizationError::Error)
    }
}

impl IAuthorization for Authorization {
    fn can_get_user(&self, actor: User, resource_id: Uuid) -> Result<bool, AuthorizationError> {
        self.is_user_admin(actor.clone())
            .or(match actor.user_id == resource_id {
                true => Ok(true),
                false => Err(AuthorizationError::Error),
            })
    }
    fn can_list_users(&self, actor: User) -> Result<bool, AuthorizationError> {
        self.is_user_admin(actor)
    }
}
