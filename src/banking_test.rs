#[cfg(test)]
mod dwolla_tests {
    use crate::banking;
    use crate::banking::BankingClient;
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

        let user_external = banking::User {
            // first_name: user.first_name.ok_or(errors::ServerError::User(
            //     anyhow!("first_name must be set"),
            //     "First name must be set".to_owned(),
            // ))?,
            // last_name: user.last_name.ok_or(errors::ServerError::User(
            //     anyhow!("last_name must be set"),
            //     "Last name must be set".to_owned(),
            // ))?,
            first_name: "first_name".to_owned(),
            last_name: "last_name".to_owned(),
            dwolla_customer_id: None,
        };

        println!("creating bank customer");
        let user_external = dwolla_client.create_customer(user_external).await?;

        assert_eq!("test", user_external.dwolla_customer_id.unwrap());
        Ok(())
    }
}
