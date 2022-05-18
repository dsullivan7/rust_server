// #[path = "services_test.rs"]
// #[cfg(test)]
// mod services_test;

use mockall::*;
use serde::Serialize;

const DAYS_IN_YEAR: f64 = 365.0;
const HOURS_IN_DAY: f64 = 24.0;

#[derive(Clone)]
pub struct Services {}

pub struct Order {
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
}

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
}
