#[path = "points_test.rs"]
#[cfg(test)]
mod points_test;

use actix_web::{get, web, Error, Responder};
use anyhow::{anyhow, Result};
use sea_orm::entity::*;
use sea_orm::QueryFilter;
use serde::Deserialize;
use uuid::Uuid;

use crate::authentication::Claims;
use crate::errors;
use crate::models;
use crate::models::point::Entity as Point;
use crate::AppState;

#[derive(Deserialize)]
struct QueryParams {
    user_id: Option<Uuid>,
}

#[get("/points")]
async fn list_points(
    data: web::Data<AppState>,
    _claims: Claims,
    query: web::Query<QueryParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let mut sql_query = sea_orm::Condition::all();

    if let Some(user_id) = &query.user_id {
        sql_query = sql_query.add(models::point::Column::UserId.eq(user_id.to_owned()));
    }

    let points: Vec<models::point::Model> = Point::find()
        .filter(sql_query)
        .all(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(points))
}

#[get("/points/{point_id}")]
async fn get_point(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let point_id = &path.into_inner();

    let conn = &data.conn;

    let point: models::point::Model = (|| -> Result<_, Error> {
        let point_id_uuid = uuid::Uuid::parse_str(point_id)
            .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;
        Ok(Point::find_by_id(point_id_uuid).one(conn))
    })()?
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
    .ok_or(errors::ServerError::NotFound)?;

    Ok(web::Json(point))
}
