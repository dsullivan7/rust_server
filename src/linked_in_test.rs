#[cfg(test)]
mod linked_in_tests {
    use crate::linked_in::LinkedInClient;
    use crate::linked_in::LinkedInError;
    use crate::linked_in::LinkedInUser;
    use crate::linked_in::ILinkedInClient;

    #[ignore]
    #[tokio::test]
    async fn test_get_user() -> Result<(), LinkedInError> {
        let linked_in_client_id =
            std::env::var("AUTH0_CLIENT_ID").expect("AUTH0_CLIENT_ID must be set");
        let linked_in_client_secret =
            std::env::var("AUTH0_CLIENT_SECRET").expect("AUTH0_CLIENT_SECRET must be set");
        let linked_in_domain = std::env::var("AUTH0_DOMAIN").expect("AUTH0_DOMAIN must be set");

        let linked_in_client = LinkedInClient::new(
            linked_in_client_id,
            linked_in_client_secret,
            format!("https://{}", linked_in_domain),
        );

        let user: LinkedInUser = linked_in_client
            .get_user("google-oauth2|107121023659381840258".to_owned())
            .await?;

        println!("identity provider");
        println!("{}", user.identities[0].provider);

        Ok(())
    }
}
