use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "bank_accounts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub bank_transfer_id: Uuid,
    pub bank_account_id: Uuid,
    pub amount: i32,
    pub dwolla_transfer_id: String,
    pub status: String,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
