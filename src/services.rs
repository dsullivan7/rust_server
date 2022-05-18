// #[path = "services_test.rs"]
// #[cfg(test)]
// mod services_test;

use mockall::*;
use serde::Serialize;

#[derive(Clone)]
pub struct Services {}

pub struct Order {
    amount: i32,
    side: String,
    status: String,
}

#[derive(Serialize)]
pub struct Balance {
    pub total: i32,
    pub interest: i32,
    pub principal: i32,
}

#[automock]
pub trait IServices: Send + Sync {
    fn get_balance(&self, orders: Vec<Order>, interest: f64) -> Balance;
}

impl IServices for Services {
    fn get_balance(&self, orders: Vec<Order>, interest: f64) -> Balance {
        return Balance {
            total: 1100,
            interest: 100,
            principal: 1000,
        };
    }
}
