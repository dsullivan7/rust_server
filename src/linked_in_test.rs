#[cfg(test)]
mod linked_in_tests {
    use crate::linked_in::LinkedInClient;
    use crate::linked_in::LinkedInError;
    use crate::linked_in::LinkedInUser;
    use crate::linked_in::ILinkedInClient;

    #[tokio::test]
    async fn test_get_user() -> Result<(), LinkedInError> {
        let access_token = "".to_owned();

        let linked_in_api_url = std::env::var("LINKED_IN_API_URL").expect("LINKED_IN_DOMAIN must be set");

        let linked_in_client = LinkedInClient::new(
            linked_in_api_url,
        );

        let user: LinkedInUser = linked_in_client
            .get_me(access_token.to_owned())
            .await?;

        println!("linked_in user id");
        println!("{}", user.id);

        Ok(())
    }
}
