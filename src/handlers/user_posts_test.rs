use actix_web::{test, App};
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};

use crate::test_utils;

#[cfg(test)]
#[tokio::test]
async fn test_get_user_post() {
    use super::*;

    let user_post_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let user_post_db: models::user_post::Model = models::user_post::Model {
        user_post_id: user_post_id.to_owned(),
        post_id: post_id.to_owned(),
        user_id: user_id.to_owned(),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![user_post_db.clone()]])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    }
    .with_default_auth();

    let state = web::Data::new(test_state.into_app_state());
    let app = test::init_service(App::new().app_data(state).service(get_user_post)).await;

    let path = format!("/user-posts/{}", user_post_id);
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
    //             r#"SELECT "user_post"."user_post_id", "user_post"."name", "user_post"."last_name", "user_post"."created_at", "user_post"."updated_at" FROM "user_posts" LIMIT $1"#,
    //             vec![1u64.into()]
    //         ),
    //     ],
    // );

    let user_post_resp: models::user_post::Model = actix_web::test::read_body_json(resp).await;

    assert_eq!(user_post_resp.user_post_id, user_post_db.user_post_id);
    assert_eq!(user_post_resp.post_id, user_post_db.post_id);
    assert_eq!(user_post_resp.user_id, user_post_db.user_id);
    assert_eq!(user_post_resp.created_at, user_post_db.created_at);
    assert_eq!(user_post_resp.updated_at, user_post_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_list_user_post() {
    use super::*;

    let user_post_id_1 = Uuid::new_v4();
    let post_id_1 = Uuid::new_v4();
    let user_id_1 = Uuid::new_v4();

    let user_post_db: models::user_post::Model = models::user_post::Model {
        user_post_id: user_post_id_1.to_owned(),
        post_id: post_id_1.to_owned(),
        user_id: user_id_1.to_owned(),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![user_post_db.clone()]])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    }
    .with_default_auth();

    let state = web::Data::new(test_state.into_app_state());

    let app = test::init_service(App::new().app_data(state).service(list_user_posts)).await;

    let req = test::TestRequest::get()
        .uri("/user-posts")
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let user_posts_resp: Vec<models::user_post::Model> =
        actix_web::test::read_body_json(resp).await;

    assert_eq!(user_posts_resp[0].user_post_id, user_post_db.user_post_id);
    assert_eq!(user_posts_resp[0].post_id, user_post_db.post_id);
    assert_eq!(user_posts_resp[0].user_id, user_post_db.user_id);
    assert_eq!(user_posts_resp[0].created_at, user_post_db.created_at);
    assert_eq!(user_posts_resp[0].updated_at, user_post_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_create_user_post() {
    use super::*;

    let user_post_id_1 = Uuid::new_v4();
    let post_id_1 = Uuid::new_v4();
    let user_id_1 = Uuid::new_v4();
    let point_id_1 = Uuid::new_v4();

    let user_post_db: models::user_post::Model = models::user_post::Model {
        user_post_id: user_post_id_1.to_owned(),
        post_id: post_id_1.to_owned(),
        user_id: user_id_1.to_owned(),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let point_db: models::point::Model = models::point::Model {
        point_id: point_id_1.to_owned(),
        user_id: user_id_1.to_owned(),
        amount: 100,
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![user_post_db.clone()]])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .append_query_results(vec![vec![point_db.clone()]])
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

    let app = test::init_service(App::new().app_data(state).service(create_user_post)).await;

    let body = serde_json::json!({
        "post_id": post_id_1,
        "user_id": user_id_1,
    });

    let req = test::TestRequest::post()
        .set_json(&body)
        .uri("/user-posts")
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::CREATED);

    let user_post_resp: models::user_post::Model = actix_web::test::read_body_json(resp).await;

    assert_eq!(user_post_resp.user_post_id, user_post_db.user_post_id);
    assert_eq!(user_post_resp.post_id, user_post_db.post_id);
    assert_eq!(user_post_resp.user_id, user_post_db.user_id);
    assert_eq!(user_post_resp.created_at, user_post_db.created_at);
    assert_eq!(user_post_resp.updated_at, user_post_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_modify_user_post() {
    use super::*;

    let user_post_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let user_post_db: models::user_post::Model = models::user_post::Model {
        user_post_id: user_post_id.to_owned(),
        post_id: post_id.to_owned(),
        user_id: user_id.to_owned(),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let user_post_db_modified: models::user_post::Model = models::user_post::Model {
        user_post_id: user_post_id.to_owned(),
        post_id: post_id.to_owned(),
        user_id: user_id.to_owned(),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![user_post_db.clone()]])
        .append_query_results(vec![vec![user_post_db_modified.clone()]])
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

    let app = test::init_service(App::new().app_data(state).service(modify_user_post)).await;

    let body = serde_json::json!({});

    let path = format!("/user-posts/{}", user_post_id);
    let req = test::TestRequest::put()
        .set_json(&body)
        .uri(&path)
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let user_post_resp: models::user_post::Model = actix_web::test::read_body_json(resp).await;

    assert_eq!(
        user_post_resp.user_post_id,
        user_post_db_modified.user_post_id
    );
    assert_eq!(user_post_resp.post_id, user_post_db_modified.post_id);
    assert_eq!(user_post_resp.user_id, user_post_db_modified.user_id);
    assert_eq!(user_post_resp.created_at, user_post_db_modified.created_at);
    assert_eq!(user_post_resp.updated_at, user_post_db_modified.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_delete_user_post() {
    use super::*;

    let user_post_id = Uuid::new_v4();

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

    let app = test::init_service(App::new().app_data(state).service(delete_user_post)).await;

    let path = format!("/user-posts/{}", user_post_id);
    let req = test::TestRequest::delete()
        .uri(&path)
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);
}
