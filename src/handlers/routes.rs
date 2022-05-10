use actix_web::{web, Scope};

use super::balances;
use super::bank_accounts;
use super::bank_transfers;
use super::plaid;
use super::users;

pub fn routes() -> Scope {
    web::scope("/api")
        .service(users::get_user)
        .service(users::list_users)
        .service(users::create_user)
        .service(users::modify_user)
        .service(users::delete_user)
        .service(bank_accounts::get_bank_account)
        .service(bank_accounts::list_bank_accounts)
        .service(bank_accounts::create_bank_account)
        .service(bank_accounts::modify_bank_account)
        .service(bank_accounts::delete_bank_account)
        .service(bank_transfers::get_bank_transfer)
        .service(bank_transfers::list_bank_transfers)
        .service(bank_transfers::create_bank_transfer)
        .service(bank_transfers::modify_bank_transfer)
        .service(bank_transfers::delete_bank_transfer)
        .service(balances::get_balances)
        .service(plaid::create_token)
}
