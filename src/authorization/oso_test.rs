#[cfg(test)]
mod test_authorization {
    use crate::authorization;
    use crate::authorization::oso::OsoAuthorizationClient;
    use crate::authorization::AuthorizationClient;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_allow_user_action_field() {
        let oso_authz_client = OsoAuthorizationClient::new();

        let actor = authorization::User {
            user_id: Uuid::new_v4(),
            role: "user".to_owned(),
        };

        let resource = authorization::User {
            user_id: Uuid::new_v4(),
            role: "user".to_owned(),
        };

        let result = oso_authz_client.allow_user_action_field(
            actor,
            "update".to_owned(),
            resource,
            "first_name".to_owned(),
        );

        assert!(result);
    }
}
