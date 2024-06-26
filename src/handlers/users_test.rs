use actix_web::{test, App};
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use uuid::Uuid;

use crate::test_utils;

#[cfg(test)]
#[tokio::test]
async fn test_get_user() {
    use super::*;

    let user_id = Uuid::new_v4();

    let user_db: models::user::Model = models::user::Model {
        user_id: user_id.to_owned(),
        first_name: Some("first_name".to_owned()),
        last_name: Some("last_name".to_owned()),
        auth0_id: Some("auth0_id".to_owned()),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![user_db.clone()]])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    }
    .with_default_auth();

    let state = web::Data::new(test_state.into_app_state());
    let app = test::init_service(App::new().app_data(state).service(get_user)).await;

    let path = format!("/users/{}", user_id);
    let req = test::TestRequest::get()
        .uri(&path)
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    // assert_eq!(
    //     db.into_transaction_log(),
    //     vec![
    //         Transaction::from_sql_and_values(
    //             DatabaseBackend::Postgres,
    //             r#"SELECT "user"."user_id", "user"."first_name", "user"."last_name", "user"."created_at", "user"."updated_at" FROM "users" LIMIT $1"#,
    //             vec![1u64.into()]
    //         ),
    //     ],
    // );

    let user_resp: models::user::Model = actix_web::test::read_body_json(resp).await;

    assert_eq!(user_resp.user_id, user_db.user_id);
    assert_eq!(user_resp.first_name, user_db.first_name);
    assert_eq!(user_resp.last_name, user_db.last_name);
    assert_eq!(user_resp.auth0_id, user_db.auth0_id);
    assert_eq!(user_resp.created_at, user_db.created_at);
    assert_eq!(user_resp.updated_at, user_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_list_user() {
    use super::*;

    let user_id_1 = Uuid::new_v4();

    let user_db: models::user::Model = models::user::Model {
        user_id: user_id_1.to_owned(),
        first_name: Some("first_name".to_owned()),
        last_name: Some("last_name".to_owned()),
        auth0_id: Some("auth0_id".to_owned()),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![user_db.clone()]])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    }
    .with_default_auth();

    let state = web::Data::new(test_state.into_app_state());

    let app = test::init_service(App::new().app_data(state).service(list_users)).await;

    let req = test::TestRequest::get()
        .uri("/users")
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let users_resp: Vec<models::user::Model> = actix_web::test::read_body_json(resp).await;

    assert_eq!(users_resp[0].user_id, user_db.user_id);
    assert_eq!(users_resp[0].first_name, user_db.first_name);
    assert_eq!(users_resp[0].last_name, user_db.last_name);
    assert_eq!(users_resp[0].auth0_id, user_db.auth0_id);
    assert_eq!(users_resp[0].created_at, user_db.created_at);
    assert_eq!(users_resp[0].updated_at, user_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_create_user() {
    use super::*;

    let user_id_1 = Uuid::new_v4();

    let user_db: models::user::Model = models::user::Model {
        user_id: user_id_1.to_owned(),
        first_name: Some("first_name".to_owned()),
        last_name: Some("last_name".to_owned()),
        auth0_id: Some("auth0_id".to_owned()),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![Vec::<models::user::Model>::new()])
        .append_query_results(vec![vec![user_db.clone()]])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    }
    .with_default_auth();

    let state = web::Data::new(test_state.into_app_state());

    let app = test::init_service(App::new().app_data(state).service(create_user)).await;

    let body = serde_json::json!({
        "first_name": "first_name",
        "last_name": "last_name",
        "auth0_id": "auth0_id",
    });

    let req = test::TestRequest::post()
        .set_json(&body)
        .uri("/users")
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::CREATED);

    let user_resp: models::user::Model = actix_web::test::read_body_json(resp).await;

    assert_eq!(user_resp.user_id, user_db.user_id);
    assert_eq!(user_resp.first_name, user_db.first_name);
    assert_eq!(user_resp.last_name, user_db.last_name);
    assert_eq!(user_resp.auth0_id, user_db.auth0_id);
    assert_eq!(user_resp.created_at, user_db.created_at);
    assert_eq!(user_resp.updated_at, user_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_modify_user() {
    use super::*;

    let user_id = Uuid::new_v4();

    let user_db: models::user::Model = models::user::Model {
        user_id: user_id.to_owned(),
        first_name: Some("first_name".to_owned()),
        last_name: Some("last_name".to_owned()),
        auth0_id: Some("auth0_id".to_owned()),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let user_db_modified: models::user::Model = models::user::Model {
        user_id: user_id.to_owned(),
        first_name: Some("first_name_different".to_owned()),
        last_name: Some("last_name_different".to_owned()),
        auth0_id: Some("auth0_id".to_owned()),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![user_db.clone()]])
        .append_query_results(vec![vec![user_db_modified.clone()]])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    }
    .with_default_auth();

    let state = web::Data::new(test_state.into_app_state());

    let app = test::init_service(App::new().app_data(state).service(modify_user)).await;

    let body = serde_json::json!({
        "first_name": "first_name_different",
        "last_name": "last_name_different",
    });

    let path = format!("/users/{}", user_id);
    let req = test::TestRequest::put()
        .set_json(&body)
        .uri(&path)
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let user_resp: models::user::Model = actix_web::test::read_body_json(resp).await;

    assert_eq!(user_resp.user_id, user_db_modified.user_id);
    assert_eq!(user_resp.first_name, user_db_modified.first_name);
    assert_eq!(user_resp.last_name, user_db_modified.last_name);
    assert_eq!(user_resp.auth0_id, user_db_modified.auth0_id);
    assert_eq!(user_resp.created_at, user_db_modified.created_at);
    assert_eq!(user_resp.updated_at, user_db_modified.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_delete_user() {
    use super::*;

    let user_id = Uuid::new_v4();

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    }
    .with_default_auth();

    let state = web::Data::new(test_state.into_app_state());

    let app = test::init_service(App::new().app_data(state).service(delete_user)).await;

    let path = format!("/users/{}", user_id);
    let req = test::TestRequest::delete()
        .uri(&path)
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);
}
