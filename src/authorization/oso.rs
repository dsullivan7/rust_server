#[path = "oso_test.rs"]
#[cfg(test)]
mod oso_test;

use super::AuthorizationClient;
use super::AuthorizationError;

use anyhow::anyhow;
use oso;
use oso::PolarClass;

#[derive(Clone, PolarClass)]
pub struct User {
    #[polar(attribute)]
    user_id: String,
    #[polar(attribute)]
    role: String,
}

pub struct OsoAuthorizationClient {
    oso: oso::Oso,
}

impl OsoAuthorizationClient {
    pub fn new(oso: oso::Oso) -> OsoAuthorizationClient {
        OsoAuthorizationClient { oso }
    }
}

impl AuthorizationClient<User> for OsoAuthorizationClient {
    fn allow_user_action_field(
        &self,
        actor: User,
        action: String,
        resource: User,
        field: String,
    ) -> Result<bool, AuthorizationError> {
        let mut query = self
            .oso
            .query_rule("allow_field", (actor, action, resource, field))
            .map_err(|err| AuthorizationError::Error(anyhow!(err)))?;

        match query.next() {
            Some(Ok(_)) => Ok(true),
            Some(Err(e)) => Err(AuthorizationError::Error(anyhow!(e))),
            None => Ok(false),
        }
    }
}
