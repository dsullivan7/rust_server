#[cfg(test)]
mod auth0_tests {
    use crate::auth0::Auth0Client;
    use crate::auth0::Auth0Error;
    use crate::auth0::Auth0User;
    use crate::auth0::IAuth0Client;

    #[tokio::test]
    async fn test_get_user() -> Result<(), Auth0Error> {
        let auth0_client_id =
            std::env::var("AUTH0_CLIENT_ID").expect("AUTH0_CLIENT_ID must be set");
        let auth0_client_secret =
            std::env::var("AUTH0_CLIENT_SECRET").expect("AUTH0_CLIENT_SECRET must be set");
        let auth0_domain = std::env::var("AUTH0_DOMAIN").expect("AUTH0_DOMAIN must be set");

        let auth0_client = Auth0Client::new(
            auth0_client_id,
            auth0_client_secret,
            format!("https://{}", auth0_domain),
        );

        let access_token = auth0_client.get_access_token().await?;

        let user: Auth0User = auth0_client
            .get_user(access_token, "linkedin|ZKdNjriNNl".to_owned())
            .await?;

        if let Some(access_token) = &user.identities[0].access_token {
            println!("access_token: {}", access_token);
        } else {
            println!("no access_token found");
        }

        Ok(())
    }
}
