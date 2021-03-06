use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "bank_accounts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub bank_account_id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: Option<String>,
    pub plaid_account_id: Option<String>,
    pub plaid_access_token: Option<String>,
    pub dwolla_funding_source_id: Option<String>,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
