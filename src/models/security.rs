use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "securities")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub security_id: Uuid,
    pub symbol: String,
    pub name: String,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub created_at: chrono::NaiveDateTime,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
