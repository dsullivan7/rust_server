#[cfg(test)]
mod linked_in_tests {
    use crate::linked_in::LinkedInClient;
    use crate::linked_in::LinkedInError;
    use crate::linked_in::LinkedInUser;
    use crate::linked_in::ILinkedInClient;

    use crate::auth0::Auth0Client;
    use crate::auth0::Auth0User;
    use crate::auth0::IAuth0Client;

    #[tokio::test]
    async fn test_get_user() -> Result<(), LinkedInError> {
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

        let auth0_user: Auth0User = auth0_client
            .get_user("linkedin|ZKdNjriNNl".to_owned())
            .await.unwrap();

        // let access_token = &auth0_user.identities[0].access_token;

        // let linked_in_api_url = std::env::var("LINKED_IN_API_URL").expect("LINKED_IN_DOMAIN must be set");

        // let linked_in_client = LinkedInClient::new(
        //     linked_in_api_url,
        // );

        // let user: LinkedInUser = linked_in_client
        //     .get_me(access_token.to_owned())
        //     .await?;

        // println!("linked_in user id");
        // println!("{}", user.id);

        Ok(())
    }
}
