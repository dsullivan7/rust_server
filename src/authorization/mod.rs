use async_trait::async_trait;
use mockall::*;
use uuid::Uuid;

pub mod oso;

#[derive(Clone)]
pub struct User {
    pub user_id: Uuid,
    pub role: String,
}

#[automock]
#[async_trait]
pub trait AuthorizationClient {
    fn allow_user_action_field(
        &self,
        actor: User,
        action: String,
        resource: User,
        field: String,
    ) -> bool;
}
