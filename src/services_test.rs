#[cfg(test)]
mod tests {
    use crate::models;
    use crate::services;
    use uuid::Uuid;

    #[test]
    fn test_list_portfolio_holdings() {
        let portfolio_id = Uuid::new_v4();
        let tag_id_1 = Uuid::new_v4();
        let tag_id_2 = Uuid::new_v4();
        let security_id_1 = Uuid::new_v4();
        let security_id_2 = Uuid::new_v4();

        let portfolio = models::portfolio::Model {
            user_id: Uuid::new_v4(),
            portfolio_id: portfolio_id,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        let portfolio_tags = [
            models::portfolio_tag::Model {
                portfolio_tag_id: Uuid::new_v4(),
                portfolio_id: portfolio_id,
                tag_id: tag_id_1,
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            },
            models::portfolio_tag::Model {
                portfolio_tag_id: Uuid::new_v4(),
                portfolio_id: portfolio_id,
                tag_id: tag_id_2,
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            },
        ];

        let securities = [
            models::security::Model {
                security_id: security_id_1,
                name: String::from("name_1"),
                symbol: String::from("symbol_1"),
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            },
            models::security::Model {
                security_id: security_id_2,
                name: String::from("name_2"),
                symbol: String::from("symbol_2"),
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            },
        ];

        let security_tags = [
            models::security_tag::Model {
                security_tag_id: Uuid::new_v4(),
                security_id: security_id_1,
                tag_id: tag_id_1,
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            },
            models::security_tag::Model {
                security_tag_id: Uuid::new_v4(),
                security_id: security_id_2,
                tag_id: tag_id_2,
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            },
        ];

        let mut target = vec![
            services::PortfolioHolding {
                security_id: security_id_1,
                amount: 0.5,
            },
            services::PortfolioHolding {
                security_id: security_id_2,
                amount: 0.5,
            },
        ];

        let mut result = services::list_portfolio_holdings(
            portfolio,
            &portfolio_tags,
            &securities,
            &security_tags,
        );

        target.sort_by_key(|k| k.security_id);
        result.sort_by_key(|k| k.security_id);

        assert_eq!(&target, &result);
    }
}
