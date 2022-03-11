use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "portfolios")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub portfolio_id: Uuid,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    #[serde(with = "utils::date_format")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    #[serde(with = "utils::date_format")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
