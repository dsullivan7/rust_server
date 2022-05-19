#[cfg(test)]
mod gov_tests {
    use crate::gov::Government;
    use crate::gov::IGovernment;

    // #[ignore]
    #[tokio::test]
    async fn test_get_profile() {
        let gov_client = Government::new();
        let profile = gov_client
            .get_profile(
                "username".to_owned(),
                "password".to_owned(),
                "someipaddress".to_owned(),
                "connectebt".to_owned(),
            )
            .await;

        assert_eq!(profile.unwrap().ebt_snap_balance, "");
    }
}
