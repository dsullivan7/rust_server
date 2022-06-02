#[cfg(test)]
mod services_tests {
    use crate::services;
    use crate::services::IServices;
    use uuid::Uuid;

    #[test]
    fn test_get_orders_simple() {
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

        let buy_order = &orders[0];

        assert!(buy_order.parent_order_id.is_some());
        assert_eq!(buy_order.parent_order_id.unwrap(), order_id_1);
        assert_eq!(buy_order.amount, 10);
        assert_eq!(buy_order.side, "buy");
        assert_eq!(buy_order.status, "complete");

        let sell_order = &orders[1];

        assert!(sell_order.parent_order_id.is_some());
        assert_eq!(sell_order.parent_order_id.unwrap(), order_id_2);
        assert_eq!(sell_order.amount, 10);
        assert_eq!(sell_order.side, "sell");
        assert_eq!(sell_order.status, "complete");
    }

    #[test]
    fn test_get_orders_multiple() {
        let services = services::Services {};

        let order_id_1 = Uuid::new_v4();
        let order_id_2 = Uuid::new_v4();
        let order_id_3 = Uuid::new_v4();
        let order_id_4 = Uuid::new_v4();

        let open_orders = vec![
            services::Order {
                order_id: order_id_1,
                parent_order_id: None,
                amount: 10,
                side: "buy".to_owned(),
                status: "pending".to_owned(),
                completed_at: None,
            },
            services::Order {
                order_id: order_id_2,
                parent_order_id: None,
                amount: 20,
                side: "buy".to_owned(),
                status: "pending".to_owned(),
                completed_at: None,
            },
            services::Order {
                order_id: order_id_3,
                parent_order_id: None,
                amount: 20,
                side: "sell".to_owned(),
                status: "pending".to_owned(),
                completed_at: None,
            },
            services::Order {
                order_id: order_id_4,
                parent_order_id: None,
                amount: 10,
                side: "sell".to_owned(),
                status: "pending".to_owned(),
                completed_at: None,
            },
        ];

        let child_orders = Vec::new();

        let orders = services.get_orders(open_orders, child_orders);

        assert_eq!(orders.len(), 6);

        let order_1 = &orders[0];

        assert!(order_1.parent_order_id.is_some());
        assert_eq!(order_1.parent_order_id.unwrap(), order_id_1);
        assert_eq!(order_1.amount, 10);
        assert_eq!(order_1.side, "buy");
        assert_eq!(order_1.status, "complete");

        let order_2 = &orders[1];

        assert!(order_2.parent_order_id.is_some());
        assert_eq!(order_2.parent_order_id.unwrap(), order_id_3);
        assert_eq!(order_2.amount, 10);
        assert_eq!(order_2.side, "sell");
        assert_eq!(order_2.status, "complete");

        let order_3 = &orders[2];

        assert!(order_3.parent_order_id.is_some());
        assert_eq!(order_3.parent_order_id.unwrap(), order_id_2);
        assert_eq!(order_3.amount, 10);
        assert_eq!(order_3.side, "buy");
        assert_eq!(order_3.status, "complete");

        let order_4 = &orders[3];

        assert!(order_4.parent_order_id.is_some());
        assert_eq!(order_4.parent_order_id.unwrap(), order_id_3);
        assert_eq!(order_4.amount, 10);
        assert_eq!(order_4.side, "sell");
        assert_eq!(order_4.status, "complete");

        let order_5 = &orders[4];

        assert!(order_5.parent_order_id.is_some());
        assert_eq!(order_5.parent_order_id.unwrap(), order_id_2);
        assert_eq!(order_5.amount, 10);
        assert_eq!(order_5.side, "buy");
        assert_eq!(order_5.status, "complete");

        let order_6 = &orders[5];

        assert!(order_6.parent_order_id.is_some());
        assert_eq!(order_6.parent_order_id.unwrap(), order_id_4);
        assert_eq!(order_6.amount, 10);
        assert_eq!(order_6.side, "sell");
        assert_eq!(order_6.status, "complete");
    }
}
