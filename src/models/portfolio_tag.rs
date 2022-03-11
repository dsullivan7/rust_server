use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "portfolio_tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub portfolio_tag_id: Uuid,
    pub portfolio_id: Uuid,
    pub tag_id: Uuid,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub created_at: chrono::NaiveDateTime,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
