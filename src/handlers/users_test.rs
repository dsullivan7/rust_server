use actix_web::{test, App};
use sea_orm::{DatabaseBackend, MockDatabase};
use uuid::Uuid;

#[cfg(test)]
#[actix_web::test]
async fn test_get_user() {
    use super::*;
    let app = test::init_service(App::new().service(get_user)).await;

    let req = test::TestRequest::get().uri("/users/blah").to_request();
    let resp = test::call_service(&app, req).await;
    let body = test::read_body(resp).await;
    assert_eq!(body, actix_web::web::Bytes::from("get user blah"));
}

#[cfg(test)]
#[actix_web::test]
async fn test_list_user() {
    use super::*;

    let user_id_1 = Uuid::new_v4();

    let user_db: models::user::Model = models::user::Model {
        user_id: user_id_1.to_owned(),
        first_name: "first_name".to_owned(),
        last_name: "last_name".to_owned(),
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![user_db.clone()]])
        .into_connection();
    let state = web::Data::new(AppState { db });
    let app = test::init_service(App::new().app_data(state).service(list_users)).await;

    let req = test::TestRequest::get().uri("/users").to_request();
    let resp = test::call_service(&app, req).await;
    let users_resp: Vec<models::user::Model> = actix_web::test::read_body_json(resp).await;

    assert_eq!(users_resp[0].user_id, user_db.user_id);
    assert_eq!(users_resp[0].first_name, user_db.first_name);
    assert_eq!(users_resp[0].last_name, user_db.last_name);
    assert_eq!(users_resp[0].created_at, user_db.created_at);
    assert_eq!(users_resp[0].updated_at, user_db.updated_at);
}

#[cfg(test)]
#[actix_web::test]
async fn test_create_user() {
    use super::*;

    let user_id_1 = Uuid::new_v4();

    let user_db: models::user::Model = models::user::Model {
        user_id: user_id_1.to_owned(),
        first_name: "first_name".to_owned(),
        last_name: "last_name".to_owned(),
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![user_db.clone()]])
        .into_connection();
    let state = web::Data::new(AppState { db });
    let app = test::init_service(App::new().app_data(state).service(create_user)).await;

    let req = test::TestRequest::post().uri("/users").to_request();
    let resp = test::call_service(&app, req).await;
    let user_resp: models::user::Model = actix_web::test::read_body_json(resp).await;

    assert_eq!(user_resp.user_id, user_db.user_id);
    assert_eq!(user_resp.first_name, user_db.first_name);
    assert_eq!(user_resp.last_name, user_db.last_name);
    assert_eq!(user_resp.created_at, user_db.created_at);
    assert_eq!(user_resp.updated_at, user_db.updated_at);
}
