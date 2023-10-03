use actix_web::{web, Scope};

use super::group_users;
use super::groups;
use super::users;

pub fn routes() -> Scope {
    web::scope("/api")
        .service(users::get_user)
        .service(users::list_users)
        .service(users::create_user)
        .service(users::modify_user)
        .service(users::delete_user)
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
}
