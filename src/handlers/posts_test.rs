use actix_web::{test, App};
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};

use crate::test_utils;

#[cfg(test)]
#[tokio::test]
async fn test_get_post() {
    use super::*;

    let post_id = Uuid::new_v4();
    let group_id = Uuid::new_v4();

    let post_db: models::post::Model = models::post::Model {
        post_id: post_id.to_owned(),
        group_id: group_id.to_owned(),
        name: "name".to_owned(),
        message: "message".to_owned(),
        url: "url".to_owned(),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![post_db.clone()]])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    }
    .with_default_auth();

    let state = web::Data::new(test_state.into_app_state());
    let app = test::init_service(App::new().app_data(state).service(get_post)).await;

    let path = format!("/posts/{}", post_id);
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
    //             r#"SELECT "post"."post_id", "post"."name", "post"."last_name", "post"."created_at", "post"."updated_at" FROM "posts" LIMIT $1"#,
    //             vec![1u64.into()]
    //         ),
    //     ],
    // );

    let post_resp: models::post::Model = actix_web::test::read_body_json(resp).await;

    assert_eq!(post_resp.post_id, post_db.post_id);
    assert_eq!(post_resp.group_id, post_db.group_id);
    assert_eq!(post_resp.name, post_db.name);
    assert_eq!(post_resp.message, post_db.message);
    assert_eq!(post_resp.url, post_db.url);
    assert_eq!(post_resp.created_at, post_db.created_at);
    assert_eq!(post_resp.updated_at, post_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_list_post() {
    use super::*;

    let post_id_1 = Uuid::new_v4();
    let group_id_1 = Uuid::new_v4();

    let post_db: models::post::Model = models::post::Model {
        post_id: post_id_1.to_owned(),
        group_id: group_id_1.to_owned(),
        name: "name".to_owned(),
        message: "message".to_owned(),
        url: "url".to_owned(),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![post_db.clone()]])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    }
    .with_default_auth();

    let state = web::Data::new(test_state.into_app_state());

    let app = test::init_service(App::new().app_data(state).service(list_posts)).await;

    let req = test::TestRequest::get()
        .uri("/posts")
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let posts_resp: Vec<models::post::Model> = actix_web::test::read_body_json(resp).await;

    assert_eq!(posts_resp[0].post_id, post_db.post_id);
    assert_eq!(posts_resp[0].group_id, post_db.group_id);
    assert_eq!(posts_resp[0].name, post_db.name);
    assert_eq!(posts_resp[0].message, post_db.message);
    assert_eq!(posts_resp[0].url, post_db.url);
    assert_eq!(posts_resp[0].created_at, post_db.created_at);
    assert_eq!(posts_resp[0].updated_at, post_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_create_post() {
    use super::*;

    let post_id_1 = Uuid::new_v4();
    let group_id_1 = Uuid::new_v4();

    let post_db: models::post::Model = models::post::Model {
        post_id: post_id_1.to_owned(),
        group_id: group_id_1.to_owned(),
        name: "name".to_owned(),
        message: "message".to_owned(),
        url: "url".to_owned(),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![post_db.clone()]])
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

    let app = test::init_service(App::new().app_data(state).service(create_post)).await;

    let body = serde_json::json!({
        "group_id": group_id_1,
        "name": "name",
        "message": "message",
        "url": "url",
    });

    let req = test::TestRequest::post()
        .set_json(&body)
        .uri("/posts")
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::CREATED);

    let post_resp: models::post::Model = actix_web::test::read_body_json(resp).await;

    assert_eq!(post_resp.post_id, post_db.post_id);
    assert_eq!(post_resp.group_id, post_db.group_id);
    assert_eq!(post_resp.name, post_db.name);
    assert_eq!(post_resp.message, post_db.message);
    assert_eq!(post_resp.url, post_db.url);
    assert_eq!(post_resp.created_at, post_db.created_at);
    assert_eq!(post_resp.updated_at, post_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_modify_post() {
    use super::*;

    let post_id = Uuid::new_v4();
    let group_id = Uuid::new_v4();

    let post_db: models::post::Model = models::post::Model {
        post_id: post_id.to_owned(),
        group_id: group_id.to_owned(),
        name: "name".to_owned(),
        message: "message".to_owned(),
        url: "url".to_owned(),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let post_db_modified: models::post::Model = models::post::Model {
        post_id: post_id.to_owned(),
        group_id: group_id.to_owned(),
        name: "name_different".to_owned(),
        message: "message_different".to_owned(),
        url: "url_different".to_owned(),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![post_db.clone()]])
        .append_query_results(vec![vec![post_db_modified.clone()]])
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

    let app = test::init_service(App::new().app_data(state).service(modify_post)).await;

    let body = serde_json::json!({
        "name": "name_different",
        "message": "message_different",
        "url": "url_different",
    });

    let path = format!("/posts/{}", post_id);
    let req = test::TestRequest::put()
        .set_json(&body)
        .uri(&path)
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let post_resp: models::post::Model = actix_web::test::read_body_json(resp).await;

    assert_eq!(post_resp.post_id, post_db_modified.post_id);
    assert_eq!(post_resp.group_id, post_db_modified.group_id);
    assert_eq!(post_resp.name, post_db_modified.name);
    assert_eq!(post_resp.message, post_db_modified.message);
    assert_eq!(post_resp.url, post_db_modified.url);
    assert_eq!(post_resp.created_at, post_db_modified.created_at);
    assert_eq!(post_resp.updated_at, post_db_modified.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_delete_post() {
    use super::*;

    let post_id = Uuid::new_v4();

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

    let app = test::init_service(App::new().app_data(state).service(delete_post)).await;

    let path = format!("/posts/{}", post_id);
    let req = test::TestRequest::delete()
        .uri(&path)
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);
}
