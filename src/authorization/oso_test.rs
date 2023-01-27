#[cfg(test)]
mod test_authorization {
    use anyhow::anyhow;
    use oso;
    use uuid::Uuid;

    use crate::authorization;
    use crate::authorization::oso::OsoAuthorizationClient;
    use crate::authorization::AuthorizationClient;

    #[tokio::test]
    async fn test_allow_user_action_field() -> Result<(), authorization::AuthorizationError> {
        let mut oso_client = oso::Oso::new();
        oso_client
            .load_str(
                r#"allow_field(_actor: User, action, _resource: User, field) if
        action in ["update"] and field in ["first_name"];"#,
            )
            .map_err(|err| authorization::AuthorizationError::Error(anyhow!(err)))?;

        let oso_authz_client = OsoAuthorizationClient::new(oso_client);

        let actor = authorization::oso::User {
            user_id: Uuid::new_v4().to_string(),
            role: "user".to_owned(),
        };

        let resource = authorization::oso::User {
            user_id: Uuid::new_v4().to_string(),
            role: "user".to_owned(),
        };

        let result = oso_authz_client.allow_user_action_field(
            actor,
            "update".to_owned(),
            resource,
            "first_name".to_owned(),
        )?;

        assert!(result);

        Ok(())
    }
}
