#[cfg(test)]
mod gov_tests {
    use crate::captcha::TwoCaptcha;
    use crate::gov::Government;
    use crate::gov::IGovernment;

    #[ignore]
    #[tokio::test]
    async fn test_get_profile() {
        env_logger::init();

        let two_captcha_key =
            std::env::var("TWO_CAPTCHA_KEY").expect("TWO_CAPTCHA_KEY must be set");

        let captcha = TwoCaptcha::new(two_captcha_key);
        let gov_client = Government::new(Box::new(captcha));
        let response = gov_client
            .get_profile(
                "username".to_owned(),
                "password".to_owned(),
                "71.167.248.83".to_owned(),
                "connectebt".to_owned(),
            )
            .await;

        let profile = response.unwrap();

        assert_eq!(profile.ebt_food_stamp_balance, "$455.97");
        assert_eq!(profile.ebt_cash_balance, "$2.37");
    }
}
