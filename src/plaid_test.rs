#[cfg(test)]
mod plaid_tests {
    use crate::plaid::IPlaidClient;
    use crate::plaid::PlaidClient;

    #[ignore]
    #[tokio::test]
    async fn test_create_token() {
        let plaid_client_id =
            std::env::var("PLAID_CLIENT_ID").expect("PLAID_CLIENT_ID must be set");
        let plaid_secret = std::env::var("PLAID_SECRET").expect("PLAID_SECRET must be set");
        let plaid_api_url = std::env::var("PLAID_API_URL").expect("PLAID_API_URL must be set");
        let plaid_redirect_uri =
            std::env::var("PLAID_REDIRECT_URI").expect("PLAID_REDIRECT_URI must be set");

        let plaid_client = PlaidClient::new(
            plaid_client_id,
            plaid_secret,
            plaid_api_url,
            plaid_redirect_uri,
        );
        let token = plaid_client.create_token("123".to_string()).await;

        assert_eq!("test", token);
    }
}
