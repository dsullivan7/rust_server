#[path = "oso_test.rs"]
#[cfg(test)]
mod oso_test;

use super::AuthorizationClient;
use super::User;

pub struct OsoAuthorizationClient {}

impl OsoAuthorizationClient {
    pub fn new() -> OsoAuthorizationClient {
        OsoAuthorizationClient {}
    }
}

impl AuthorizationClient for OsoAuthorizationClient {
    fn allow_user_action_field(
        &self,
        actor: User,
        action: String,
        resource: User,
        field: String,
    ) -> bool {
        true
    }
}
