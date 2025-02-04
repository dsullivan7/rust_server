use mockall::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone)]
pub struct User {
    pub user_id: Uuid,
    pub role: String,
}

#[automock]
pub trait IAuthorization: Send + Sync {
    fn can_get_user(&self, actor: User, resource_id: Uuid) -> bool;
    // fn can_modify_user(&self, actor: User, resource_id: Uuid) -> Result<bool, AuthorizationError>;
    // fn can_delete_user(&self, actor: User, resource_id: Uuid) -> Result<bool, AuthorizationError>;
    fn can_list_users(&self, actor: User) -> bool;
    // fn can_create_user(&self, actor: User) -> Result<bool, AuthorizationError>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Authorization;

impl Authorization {
    fn is_user_admin(&self, actor: User) -> bool {
        return actor.role == "admin";
    }
}

impl IAuthorization for Authorization {
    fn can_get_user(&self, actor: User, resource_id: Uuid) -> bool {
        return self.is_user_admin(actor.clone()) || actor.user_id == resource_id;
    }

    fn can_list_users(&self, actor: User) -> bool {
        self.is_user_admin(actor)
    }
}
