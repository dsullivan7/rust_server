use std::collections::HashMap;
use uuid::Uuid;

use crate::models;

pub struct PortfolioHolding {
    security_id: Uuid,
    amount: f64,
}

fn list_portfolio_holdings(
    _portfolio: models::portfolio::Model,
    portfolio_tags: &[models::portfolio_tag::Model],
    securities: &[models::security::Model],
    security_tags: &[models::security_tag::Model],
) -> Vec<PortfolioHolding> {
    let mut security_map = HashMap::new();
    for security in securities.iter() {
        security_map.insert(security.security_id, security);
    }
    let mut total_weight = 0;

    let mut security_weight_map = HashMap::new();
    for portfolio_tag in portfolio_tags.iter() {
        for security_tag in security_tags.iter() {
            if portfolio_tag.tag_id == security_tag.tag_id {
                let security_id = security_tag.security_id;
                if !security_weight_map.contains_key(&security_id) {
                    security_weight_map.insert(security_id, 0);
                }
                security_weight_map.insert(security_id, security_weight_map[&security_id] + 1);
                total_weight += 1;
            }
        }
    }

    let mut portfolio_holdings = Vec::new();
    let holding_num = security_weight_map.keys().len();
    let mut iter = 1;
    let mut holdings_total = 0.0;

    for (security_id, weight) in security_weight_map {
        let mut amount = ((weight as f64 / total_weight as f64) * 1000.0).round() / 1000.0;
        if iter == holding_num {
            amount = ((1.0 - holdings_total as f64) * 1000.0).round() / 1000.0;
        }
        holdings_total += amount;
        portfolio_holdings.push(PortfolioHolding {
            security_id: security_id,
            amount: ((weight as f64 / total_weight as f64) * 1000.0).round() / 1000.0,
        });
        iter += 1;
    }

    portfolio_holdings
}
