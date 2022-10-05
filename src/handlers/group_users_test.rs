use actix_web::{test, App};
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};

use crate::test_utils;

#[cfg(test)]
#[tokio::test]
async fn test_get_group_user() {
    use super::*;

    let group_user_id = Uuid::new_v4();
    let group_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let group_user_db: models::group_user::Model = models::group_user::Model {
        group_user_id: group_user_id.to_owned(),
        group_id: group_id.to_owned(),
        user_id: user_id.to_owned(),
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![group_user_db.clone()]])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    }
    .with_default_auth();

    let state = web::Data::new(test_state.into_app_state());
    let app = test::init_service(App::new().app_data(state).service(get_group_user)).await;

    let path = format!("/group-users/{}", group_user_id);
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
    //             r#"SELECT "group_user"."group_user_id", "group_user"."name", "group_user"."last_name", "group_user"."created_at", "group_user"."updated_at" FROM "group_users" LIMIT $1"#,
    //             vec![1u64.into()]
    //         ),
    //     ],
    // );

    let group_user_resp: models::group_user::Model = actix_web::test::read_body_json(resp).await;

    assert_eq!(group_user_resp.group_user_id, group_user_db.group_user_id);
    assert_eq!(group_user_resp.group_id, group_user_db.group_id);
    assert_eq!(group_user_resp.user_id, group_user_db.user_id);
    assert_eq!(group_user_resp.created_at, group_user_db.created_at);
    assert_eq!(group_user_resp.updated_at, group_user_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_list_group_user() {
    use super::*;

    let group_user_id_1 = Uuid::new_v4();
    let group_id_1 = Uuid::new_v4();
    let user_id_1 = Uuid::new_v4();

    let group_user_db: models::group_user::Model = models::group_user::Model {
        group_user_id: group_user_id_1.to_owned(),
        group_id: group_id_1.to_owned(),
        user_id: user_id_1.to_owned(),
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![group_user_db.clone()]])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    }
    .with_default_auth();

    let state = web::Data::new(test_state.into_app_state());

    let app = test::init_service(App::new().app_data(state).service(list_group_users)).await;

    let req = test::TestRequest::get()
        .uri("/group-users")
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let group_users_resp: Vec<models::group_user::Model> =
        actix_web::test::read_body_json(resp).await;

    assert_eq!(
        group_users_resp[0].group_user_id,
        group_user_db.group_user_id
    );
    assert_eq!(group_users_resp[0].group_id, group_user_db.group_id);
    assert_eq!(group_users_resp[0].user_id, group_user_db.user_id);
    assert_eq!(group_users_resp[0].created_at, group_user_db.created_at);
    assert_eq!(group_users_resp[0].updated_at, group_user_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_create_group_user() {
    use super::*;

    let group_user_id_1 = Uuid::new_v4();
    let group_id_1 = Uuid::new_v4();
    let user_id_1 = Uuid::new_v4();

    let group_user_db: models::group_user::Model = models::group_user::Model {
        group_user_id: group_user_id_1.to_owned(),
        group_id: group_id_1.to_owned(),
        user_id: user_id_1.to_owned(),
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![group_user_db.clone()]])
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

    let app = test::init_service(App::new().app_data(state).service(create_group_user)).await;

    let body = serde_json::json!({
        "group_id": group_id_1,
        "user_id": user_id_1,
    });

    let req = test::TestRequest::post()
        .set_json(&body)
        .uri("/group-users")
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::CREATED);

    let group_user_resp: models::group_user::Model = actix_web::test::read_body_json(resp).await;

    assert_eq!(group_user_resp.group_user_id, group_user_db.group_user_id);
    assert_eq!(group_user_resp.group_id, group_user_db.group_id);
    assert_eq!(group_user_resp.user_id, group_user_db.user_id);
    assert_eq!(group_user_resp.created_at, group_user_db.created_at);
    assert_eq!(group_user_resp.updated_at, group_user_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_modify_group_user() {
    use super::*;

    let group_user_id = Uuid::new_v4();
    let group_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let group_user_db: models::group_user::Model = models::group_user::Model {
        group_user_id: group_user_id.to_owned(),
        group_id: group_id.to_owned(),
        user_id: user_id.to_owned(),
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let group_user_db_modified: models::group_user::Model = models::group_user::Model {
        group_user_id: group_user_id.to_owned(),
        group_id: group_id.to_owned(),
        user_id: user_id.to_owned(),
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![group_user_db.clone()]])
        .append_query_results(vec![vec![group_user_db_modified.clone()]])
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

    let app = test::init_service(App::new().app_data(state).service(modify_group_user)).await;

    let body = serde_json::json!({});

    let path = format!("/group-users/{}", group_user_id);
    let req = test::TestRequest::put()
        .set_json(&body)
        .uri(&path)
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let group_user_resp: models::group_user::Model = actix_web::test::read_body_json(resp).await;

    assert_eq!(
        group_user_resp.group_user_id,
        group_user_db_modified.group_user_id
    );
    assert_eq!(group_user_resp.group_id, group_user_db_modified.group_id);
    assert_eq!(group_user_resp.user_id, group_user_db_modified.user_id);
    assert_eq!(
        group_user_resp.created_at,
        group_user_db_modified.created_at
    );
    assert_eq!(
        group_user_resp.updated_at,
        group_user_db_modified.updated_at
    );
}

#[cfg(test)]
#[tokio::test]
async fn test_delete_group_user() {
    use super::*;

    let group_user_id = Uuid::new_v4();

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

    let app = test::init_service(App::new().app_data(state).service(delete_group_user)).await;

    let path = format!("/group-users/{}", group_user_id);
    let req = test::TestRequest::delete()
        .uri(&path)
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);
}
