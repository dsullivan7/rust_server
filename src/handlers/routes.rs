use actix_web::{web, Scope};

use super::balances;
use super::bank_accounts;
use super::bank_transfers;
use super::group_users;
use super::groups;
use super::orders;
use super::plaid;
use super::points;
use super::portfolio_tags;
use super::portfolios;
use super::posts;
use super::profiles;
use super::tags;
use super::user_posts;
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
        .service(orders::get_order)
        .service(orders::list_orders)
        .service(orders::create_order)
        .service(orders::modify_order)
        .service(orders::delete_order)
        .service(groups::get_group)
        .service(groups::list_groups)
        .service(groups::create_group)
        .service(groups::modify_group)
        .service(groups::delete_group)
        .service(group_users::get_group_user)
        .service(group_users::list_group_users)
        .service(group_users::create_group_user)
        .service(group_users::modify_group_user)
        .service(group_users::delete_group_user)
        .service(posts::get_post)
        .service(posts::list_posts)
        .service(posts::create_post)
        .service(posts::modify_post)
        .service(posts::delete_post)
        .service(portfolios::get_portfolio)
        .service(portfolios::list_portfolios)
        .service(portfolios::create_portfolio)
        .service(portfolios::modify_portfolio)
        .service(portfolios::delete_portfolio)
        .service(portfolio_tags::get_portfolio_tag)
        .service(portfolio_tags::list_portfolio_tags)
        .service(portfolio_tags::create_portfolio_tag)
        .service(portfolio_tags::modify_portfolio_tag)
        .service(portfolio_tags::delete_portfolio_tag)
        .service(tags::get_tag)
        .service(tags::list_tags)
        .service(tags::create_tag)
        .service(tags::modify_tag)
        .service(tags::delete_tag)
        .service(user_posts::get_user_post)
        .service(user_posts::list_user_posts)
        .service(user_posts::create_user_post)
        .service(user_posts::modify_user_post)
        .service(user_posts::delete_user_post)
        .service(points::get_point)
        .service(points::list_points)
        .service(balances::get_balances)
        .service(profiles::create_profile)
        .service(plaid::create_token)
}
