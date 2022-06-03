#[path = "services_test.rs"]
#[cfg(test)]
mod services_test;

use mockall::*;
use serde::Serialize;
use std::cmp;
use uuid::Uuid;

const DAYS_IN_YEAR: f64 = 365.0;
const HOURS_IN_DAY: f64 = 24.0;

#[derive(Clone)]
pub struct Services {}

pub struct Order {
    pub order_id: Uuid,
    pub parent_order_id: Option<Uuid>,
    pub amount: i32,
    pub side: String,
    pub status: String,
    pub completed_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}

#[derive(Serialize)]
pub struct Balance {
    pub total: i32,
    pub interest: i32,
    pub principal: i32,
}

#[automock]
pub trait IServices: Send + Sync {
    fn get_balance(
        &self,
        orders: Vec<Order>,
        interest: f64,
        current_time: chrono::DateTime<chrono::FixedOffset>,
    ) -> Balance;
    fn get_orders(&self, parent_orders: Vec<Order>, child_orders: Vec<Order>) -> Vec<Order>;
}

impl Services {}

impl IServices for Services {
    fn get_balance(
        &self,
        orders: Vec<Order>,
        interest_rate: f64,
        current_time: chrono::DateTime<chrono::FixedOffset>,
    ) -> Balance {
        let mut principal = 0;
        let mut interest: f64 = 0.0;

        let hourly_interest = interest_rate / DAYS_IN_YEAR / HOURS_IN_DAY;

        orders.iter().for_each(|order| {
            if let Some(completed_at) = order.completed_at {
                let duration = current_time - completed_at;
                let order_interest =
                    (order.amount as f64) * (duration.num_hours() as f64) * hourly_interest;

                match order.side.as_str() {
                    "buy" => {
                        interest += order_interest;
                        principal += order.amount;
                    }
                    "sell" => {
                        interest -= order_interest;
                        principal -= order.amount;
                    }
                    _ => unreachable!(),
                }
            }
        });

        let interest = interest.round() as i32;

        return Balance {
            total: interest + principal,
            interest,
            principal,
        };
    }

    fn get_orders(&self, open_orders: Vec<Order>, child_orders: Vec<Order>) -> Vec<Order> {
        let open_orders_buy: Vec<&Order> = open_orders
            .iter()
            .filter(|order| order.side == "buy")
            .collect();
        let open_orders_sell: Vec<&Order> = open_orders
            .iter()
            .filter(|order| order.side == "sell")
            .collect();

        let mut orders = Vec::new();

        let mut i = 0;
        let mut j = 0;

        while i < open_orders_buy.len() && j < open_orders_sell.len() {
            let open_order_buy = open_orders_buy[i];
            let open_order_sell = open_orders_sell[j];

            let mut remaining_buy = open_order_buy.amount;
            let mut remaining_sell = open_order_sell.amount;

            // determine the amount remaining for the parent buy and sell orders
            child_orders.iter().chain(orders.iter()).for_each(|order| {
                if let Some(parent_order_id) = order.parent_order_id {
                    if parent_order_id == open_order_buy.order_id {
                        remaining_buy -= order.amount;
                    }
                    if parent_order_id == open_order_sell.order_id {
                        remaining_sell -= order.amount;
                    }
                }
            });

            if remaining_buy == 0 {
                i += 1;
                continue;
            }

            if remaining_sell == 0 {
                j += 1;
                continue;
            }

            let order_amount = cmp::min(remaining_buy, remaining_sell);

            let child_order_buy = Order {
                order_id: Uuid::new_v4(),
                parent_order_id: Some(open_order_buy.order_id),
                amount: order_amount,
                side: "buy".to_owned(),
                status: "complete".to_owned(),
                completed_at: Some(chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0))),
            };

            let child_order_sell = Order {
                order_id: Uuid::new_v4(),
                parent_order_id: Some(open_order_sell.order_id),
                amount: order_amount,
                side: "sell".to_owned(),
                status: "complete".to_owned(),
                completed_at: Some(chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0))),
            };

            orders.push(child_order_buy);
            orders.push(child_order_sell);
        }

        orders
    }
}
