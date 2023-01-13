use actix_web::{get, web, Error, Responder};
use anyhow::{anyhow, Result};
use sea_orm::entity::*;
use sea_orm::QueryFilter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::authentication::Claims;
use crate::errors;
use crate::models;
use crate::models::portfolio::Entity as Portfolio;
use crate::models::portfolio_tag::Entity as PortfolioTag;
use crate::models::security::Entity as Security;
use crate::models::security_tag::Entity as SecurityTag;
use crate::AppState;

const round_value: f64 = 10000.0;

#[derive(Deserialize)]
struct QueryParams {
    portfolio_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize)]
struct PortfolioHolding {
    symbol: String,
    name: String,
    description: String,
    amount: f64,
}

#[get("/portfolio-recommendations")]
async fn list_portfolio_recommendations(
    data: web::Data<AppState>,
    _claims: Claims,
    query: web::Query<QueryParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    if let Some(portfolio_id) = query.portfolio_id {
        let securities: Vec<models::security::Model> = Security::find()
            .all(conn)
            .await
            .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

        let portfolio_tags: Vec<models::portfolio_tag::Model> = PortfolioTag::find()
            .filter(
                sea_orm::Condition::all()
                    .add(models::portfolio_tag::Column::PortfolioId.eq(portfolio_id.to_owned())),
            )
            .all(conn)
            .await
            .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

        let security_tags: Vec<models::security_tag::Model> =
            SecurityTag::find()
                .all(conn)
                .await
                .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

        let mut security_map = HashMap::new();
        let mut total_weight = 0;

        portfolio_tags.iter().for_each(|portfolio_tag| {
            security_tags.iter().for_each(|security_tag| {
                if security_tag.tag_id == portfolio_tag.tag_id {
                    let security_weight = security_map.entry(security_tag.security_id).or_insert(0);
                    *security_weight += 1;
                    total_weight += 1;
                }
            })
        });

        let mut remaining = 1.0;
        let mut portfolio_holdings = vec![];
        let mut security_tuples = security_map.into_iter().collect::<Vec<(Uuid, i32)>>();
        security_tuples.sort_by(|(id1, _), (id2, _)| id1.cmp(id2));

        let total_securities = security_tuples.len();
        security_tuples
            .into_iter()
            .enumerate()
            .for_each(|(i, (security_id, weight))| {
                let security_option = securities
                    .iter()
                    .find(|security| security.security_id == security_id);

                if let Some(security_found) = security_option {
                    let amount = if i + 1 == total_securities {
                        remaining
                    } else {
                        ((weight as f64) / (total_weight as f64) * round_value).round()
                            / round_value
                    };

                    remaining -= amount;

                    portfolio_holdings.push(PortfolioHolding {
                        symbol: security_found.symbol.to_owned(),
                        name: security_found.name.to_owned(),
                        description: security_found.description.to_owned(),
                        amount,
                    });
                }
            });

        return Ok(web::Json(portfolio_holdings));
    }

    Ok(web::Json(vec![]))
}
