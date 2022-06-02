#[cfg(test)]
mod services_tests {
    use crate::services;
    use crate::services::IServices;
    use uuid::Uuid;

    #[test]
    fn test_get_orders() {
        let services = services::Services {};

        let order_id_1 = Uuid::new_v4();
        let order_id_2 = Uuid::new_v4();

        let open_orders = vec![
            services::Order {
                order_id: order_id_1,
                parent_order_id: None,
                amount: 20,
                side: "buy".to_owned(),
                status: "pending".to_owned(),
                completed_at: None,
            },
            services::Order {
                order_id: order_id_2,
                parent_order_id: None,
                amount: 10,
                side: "sell".to_owned(),
                status: "pending".to_owned(),
                completed_at: None,
            },
        ];

        let child_orders = Vec::new();

        let orders = services.get_orders(open_orders, child_orders);

        let buy_order = orders[0];

        assert!(buy_order.parent_order_id.is_some());
        assert_eq!(buy_order.parent_order_id.unwrap(), order_id_1);
        assert_eq!(buy_order.amount, 10);
        assert_eq!(buy_order.side, "buy");
        assert_eq!(buy_order.status, "complete");

        let sell_order = orders[1];

        assert!(sell_order.parent_order_id.is_some());
        assert_eq!(sell_order.parent_order_id.unwrap(), order_id_2);
        assert_eq!(sell_order.amount, 10);
        assert_eq!(sell_order.side, "sell");
        assert_eq!(sell_order.status, "complete");
    }
}
