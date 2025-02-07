use mockall::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use derive_more::Display;

#[derive(Clone)]
pub struct User {
    pub user_id: Uuid,
    pub role: String,
}

// define authorization error struct
#[derive(Debug, Display)]
pub enum AuthorizationError {
    #[display(fmt = "not authorized")]
    NotAuthorized(),
}

#[automock]
pub trait IAuthorization: Send + Sync {
    fn can_get_user(&self, actor: User, resource_id: Uuid) -> Result<(), AuthorizationError>;
    // fn can_modify_user(&self, actor: User, resource_id: Uuid) -> Result<bool, AuthorizationError>;
    // fn can_delete_user(&self, actor: User, resource_id: Uuid) -> Result<bool, AuthorizationError>;
    fn can_list_users(&self, actor: User) -> Result<(), AuthorizationError>;
    // fn can_create_user(&self, actor: User) -> Result<bool, AuthorizationError>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Authorization;

impl Authorization {
    fn is_user_admin(&self, actor: User) -> bool {
        actor.role == "admin"
    }
}

impl IAuthorization for Authorization {
    fn can_get_user(&self, actor: User, resource_id: Uuid) -> Result<(), AuthorizationError> {
        if self.is_user_admin(actor.clone()) || actor.user_id == resource_id {
            return Ok(());
        }
        Err(AuthorizationError::NotAuthorized())
    }

    fn can_list_users(&self, actor: User) -> Result<(), AuthorizationError> {
        if self.is_user_admin(actor) {
            return Ok(());
        }
        Err(AuthorizationError::NotAuthorized())
    }
}
