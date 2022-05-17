#[cfg(test)]
mod dwolla_tests {
    // use crate::banking::BankingClient;
    use crate::banking::BankingError;
    use crate::banking::DwollaClient;

    #[ignore]
    #[tokio::test]
    async fn test_authenticate() -> Result<(), BankingError> {
        let dwolla_api_key = std::env::var("DWOLLA_API_KEY").expect("DWOLLA_API_KEY must be set");
        let dwolla_api_secret =
            std::env::var("DWOLLA_API_SECRET").expect("DWOLLA_API_SECRET must be set");
        let dwolla_api_url = std::env::var("DWOLLA_API_URL").expect("DWOLLA_API_URL must be set");

        let dwolla_client = DwollaClient::new(dwolla_api_key, dwolla_api_secret, dwolla_api_url);

        dwolla_client.authenticate().await?;

        assert_eq!(
            "test",
            dwolla_client
                .api_access_token
                .read()
                .await
                .as_ref()
                .unwrap()
        );
        Ok(())
    }
}
