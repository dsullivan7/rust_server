use actix_web::{test, App};
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use uuid::Uuid;

use crate::test_utils;

#[cfg(test)]
#[tokio::test]
async fn test_get_group() {
    use super::*;

    let group_id = Uuid::new_v4();

    let group_db: models::group::Model = models::group::Model {
        group_id: group_id.to_owned(),
        name: "name".to_owned(),
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![group_db.clone()]])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    }
    .with_default_auth();

    let state = web::Data::new(test_state.into_app_state());
    let app = test::init_service(App::new().app_data(state).service(get_group)).await;

    let path = format!("/groups/{}", group_id);
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
    //             r#"SELECT "group"."group_id", "group"."name", "group"."last_name", "group"."created_at", "group"."updated_at" FROM "groups" LIMIT $1"#,
    //             vec![1u64.into()]
    //         ),
    //     ],
    // );

    let group_resp: models::group::Model = actix_web::test::read_body_json(resp).await;

    assert_eq!(group_resp.group_id, group_db.group_id);
    assert_eq!(group_resp.name, group_db.name);
    assert_eq!(group_resp.created_at, group_db.created_at);
    assert_eq!(group_resp.updated_at, group_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_list_group() {
    use super::*;

    let group_id_1 = Uuid::new_v4();

    let group_db: models::group::Model = models::group::Model {
        group_id: group_id_1.to_owned(),
        name: "name".to_owned(),
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![group_db.clone()]])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    }
    .with_default_auth();

    let state = web::Data::new(test_state.into_app_state());

    let app = test::init_service(App::new().app_data(state).service(list_groups)).await;

    let req = test::TestRequest::get()
        .uri("/groups")
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let groups_resp: Vec<models::group::Model> = actix_web::test::read_body_json(resp).await;

    assert_eq!(groups_resp[0].group_id, group_db.group_id);
    assert_eq!(groups_resp[0].name, group_db.name);
    assert_eq!(groups_resp[0].created_at, group_db.created_at);
    assert_eq!(groups_resp[0].updated_at, group_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_create_group() {
    use super::*;

    let group_id_1 = Uuid::new_v4();

    let group_db: models::group::Model = models::group::Model {
        group_id: group_id_1.to_owned(),
        name: "name".to_owned(),
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![group_db.clone()]])
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

    let app = test::init_service(App::new().app_data(state).service(create_group)).await;

    let body = serde_json::json!({
        "name": "name",
    });

    let req = test::TestRequest::post()
        .set_json(&body)
        .uri("/groups")
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::CREATED);

    let group_resp: models::group::Model = actix_web::test::read_body_json(resp).await;

    assert_eq!(group_resp.group_id, group_db.group_id);
    assert_eq!(group_resp.name, group_db.name);
    assert_eq!(group_resp.created_at, group_db.created_at);
    assert_eq!(group_resp.updated_at, group_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_modify_group() {
    use super::*;

    let group_id = Uuid::new_v4();

    let group_db: models::group::Model = models::group::Model {
        group_id: group_id.to_owned(),
        name: "name".to_owned(),
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let group_db_modified: models::group::Model = models::group::Model {
        group_id: group_id.to_owned(),
        name: "name_different".to_owned(),
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![group_db.clone()]])
        .append_query_results(vec![vec![group_db_modified.clone()]])
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

    let app = test::init_service(App::new().app_data(state).service(modify_group)).await;

    let body = serde_json::json!({
        "name": "name_different",
    });

    let path = format!("/groups/{}", group_id);
    let req = test::TestRequest::put()
        .set_json(&body)
        .uri(&path)
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let group_resp: models::group::Model = actix_web::test::read_body_json(resp).await;

    assert_eq!(group_resp.group_id, group_db_modified.group_id);
    assert_eq!(group_resp.name, group_db_modified.name);
    assert_eq!(group_resp.created_at, group_db_modified.created_at);
    assert_eq!(group_resp.updated_at, group_db_modified.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_delete_group() {
    use super::*;

    let group_id = Uuid::new_v4();

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

    let app = test::init_service(App::new().app_data(state).service(delete_group)).await;

    let path = format!("/groups/{}", group_id);
    let req = test::TestRequest::delete()
        .uri(&path)
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);
}
