#[cfg(test)]
mod captcha_tests {
    use crate::captcha::Captcha;
    use crate::captcha::TwoCaptcha;

    #[ignore]
    #[tokio::test]
    async fn test_get_profile() {
        env_logger::init();

        let two_captcha_key =
            std::env::var("TWO_CAPTCHA_KEY").expect("TWO_CAPTCHA_KEY must be set");

        let captcha_client = TwoCaptcha::new(two_captcha_key);
        let response = captcha_client
            .solve_recaptcha_v2("something".to_owned(), "someurl".to_owned())
            .await;

        assert_eq!(response.unwrap(), "");
    }
}
