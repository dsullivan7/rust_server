use mockall::*;
use thiserror::Error;

pub mod oso;

#[derive(Error, Debug)]
pub enum AuthorizationError {
    #[error("authorization error")]
    Error(anyhow::Error),
}

#[automock]
pub trait AuthorizationClient<T> {
    fn allow_user_action_field(
        &self,
        actor: T,
        action: String,
        resource: T,
        field: String,
    ) -> Result<bool, AuthorizationError>;
}
