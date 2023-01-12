use actix_web::{test, App};
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};

use crate::test_utils;

#[cfg(test)]
#[tokio::test]
async fn test_get_bank_account() {
    use super::*;

    let bank_account_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let bank_account_db: models::bank_account::Model = models::bank_account::Model {
        bank_account_id: bank_account_id.to_owned(),
        user_id: Some(user_id.to_owned()),
        name: Some("name".to_owned()),
        plaid_account_id: Some("plaid_account_id".to_owned()),
        plaid_access_token: Some("plaid_access_token".to_owned()),
        dwolla_funding_source_id: Some("dwolla_funding_source_id".to_owned()),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![bank_account_db.clone()]])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    };

    let state = web::Data::new(test_state.into_app_state());
    let app = test::init_service(App::new().app_data(state).service(get_bank_account)).await;

    let path = format!("/bank-accounts/{}", bank_account_id);
    let req = test::TestRequest::get().uri(&path).to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let bank_account_resp: models::bank_account::Model =
        actix_web::test::read_body_json(resp).await;

    assert_eq!(
        bank_account_resp.bank_account_id,
        bank_account_db.bank_account_id
    );
    assert_eq!(bank_account_resp.user_id, bank_account_db.user_id);
    assert_eq!(
        bank_account_resp.plaid_account_id,
        bank_account_db.plaid_account_id
    );
    assert_eq!(
        bank_account_resp.plaid_access_token,
        bank_account_db.plaid_access_token
    );
    assert_eq!(
        bank_account_resp.dwolla_funding_source_id,
        bank_account_db.dwolla_funding_source_id
    );
    assert_eq!(bank_account_resp.created_at, bank_account_db.created_at);
    assert_eq!(bank_account_resp.updated_at, bank_account_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_list_bank_account() {
    use super::*;

    let bank_account_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let bank_account_db: models::bank_account::Model = models::bank_account::Model {
        bank_account_id: bank_account_id.to_owned(),
        user_id: Some(user_id.to_owned()),
        name: Some("name".to_owned()),
        plaid_account_id: Some("plaid_account_id".to_owned()),
        plaid_access_token: Some("plaid_access_token".to_owned()),
        dwolla_funding_source_id: Some("dwolla_funding_source_id".to_owned()),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![bank_account_db.clone()]])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    };

    let state = web::Data::new(test_state.into_app_state());

    let app = test::init_service(App::new().app_data(state).service(list_bank_accounts)).await;

    let req = test::TestRequest::get().uri("/bank-accounts").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let bank_accounts_resp: Vec<models::bank_account::Model> =
        actix_web::test::read_body_json(resp).await;

    assert_eq!(
        bank_accounts_resp[0].bank_account_id,
        bank_account_db.bank_account_id
    );
    assert_eq!(bank_accounts_resp[0].user_id, bank_account_db.user_id);
    assert_eq!(
        bank_accounts_resp[0].plaid_account_id,
        bank_account_db.plaid_account_id
    );
    assert_eq!(
        bank_accounts_resp[0].plaid_access_token,
        bank_account_db.plaid_access_token
    );
    assert_eq!(
        bank_accounts_resp[0].dwolla_funding_source_id,
        bank_account_db.dwolla_funding_source_id
    );
    assert_eq!(bank_accounts_resp[0].created_at, bank_account_db.created_at);
    assert_eq!(bank_accounts_resp[0].updated_at, bank_account_db.updated_at);
}

#[ignore]
#[cfg(test)]
#[tokio::test]
async fn test_create_bank_account() {
    use super::*;

    let bank_account_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let bank_account_db: models::bank_account::Model = models::bank_account::Model {
        bank_account_id: bank_account_id.to_owned(),
        user_id: Some(user_id.to_owned()),
        name: Some("name".to_owned()),
        plaid_account_id: Some("plaid_account_id".to_owned()),
        plaid_access_token: Some("plaid_access_token".to_owned()),
        dwolla_funding_source_id: Some("dwolla_funding_source_id".to_owned()),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![bank_account_db.clone()]])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    };

    let state = web::Data::new(test_state.into_app_state());

    let app = test::init_service(App::new().app_data(state).service(create_bank_account)).await;

    let body = serde_json::json!({
        "user_id": user_id.to_string(),
        "plaid_public_token": "plaid_public_token",
    });

    let req = test::TestRequest::post()
        .set_json(&body)
        .uri("/bank-accounts")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::CREATED);

    let bank_account_resp: models::bank_account::Model =
        actix_web::test::read_body_json(resp).await;

    assert_eq!(
        bank_account_resp.bank_account_id,
        bank_account_db.bank_account_id
    );
    assert_eq!(bank_account_resp.user_id, bank_account_db.user_id);
    assert_eq!(
        bank_account_resp.plaid_account_id,
        bank_account_db.plaid_account_id
    );
    assert_eq!(
        bank_account_resp.plaid_access_token,
        bank_account_db.plaid_access_token
    );
    assert_eq!(
        bank_account_resp.dwolla_funding_source_id,
        bank_account_db.dwolla_funding_source_id
    );
    assert_eq!(bank_account_resp.created_at, bank_account_db.created_at);
    assert_eq!(bank_account_resp.updated_at, bank_account_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_modify_bank_account() {
    use super::*;

    let bank_account_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let bank_account_db: models::bank_account::Model = models::bank_account::Model {
        bank_account_id: bank_account_id.to_owned(),
        user_id: Some(user_id.to_owned()),
        name: Some("name".to_owned()),
        plaid_account_id: Some("plaid_account_id".to_owned()),
        plaid_access_token: Some("plaid_access_token".to_owned()),
        dwolla_funding_source_id: Some("dwolla_funding_source_id".to_owned()),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let bank_account_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let bank_account_db_modified: models::bank_account::Model = models::bank_account::Model {
        bank_account_id: bank_account_id.to_owned(),
        user_id: Some(user_id.to_owned()),
        name: Some("name".to_owned()),
        plaid_account_id: Some("plaid_account_id".to_owned()),
        plaid_access_token: Some("plaid_access_token".to_owned()),
        dwolla_funding_source_id: Some("dwolla_funding_source_id".to_owned()),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![bank_account_db.clone()]])
        .append_query_results(vec![vec![bank_account_db_modified.clone()]])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    };

    let state = web::Data::new(test_state.into_app_state());

    let app = test::init_service(App::new().app_data(state).service(modify_bank_account)).await;

    let body = serde_json::json!({
        "first_name": "first_name_different",
        "last_name": "last_name_different",
    });

    let path = format!("/bank-accounts/{}", bank_account_id);
    let req = test::TestRequest::put()
        .set_json(&body)
        .uri(&path)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let bank_account_resp: models::bank_account::Model =
        actix_web::test::read_body_json(resp).await;

    assert_eq!(
        bank_account_resp.bank_account_id,
        bank_account_db_modified.bank_account_id
    );
    assert_eq!(bank_account_resp.user_id, bank_account_db_modified.user_id);
    assert_eq!(
        bank_account_resp.plaid_account_id,
        bank_account_db_modified.plaid_account_id
    );
    assert_eq!(
        bank_account_resp.plaid_access_token,
        bank_account_db_modified.plaid_access_token
    );
    assert_eq!(
        bank_account_resp.dwolla_funding_source_id,
        bank_account_db_modified.dwolla_funding_source_id
    );
    assert_eq!(
        bank_account_resp.created_at,
        bank_account_db_modified.created_at
    );
    assert_eq!(
        bank_account_resp.updated_at,
        bank_account_db_modified.updated_at
    );
}

#[cfg(test)]
#[tokio::test]
async fn test_delete_bank_account() {
    use super::*;

    let bank_account_id = Uuid::new_v4();

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    };

    let state = web::Data::new(test_state.into_app_state());

    let app = test::init_service(App::new().app_data(state).service(delete_bank_account)).await;

    let path = format!("/bank-accounts/{}", bank_account_id);
    let req = test::TestRequest::delete().uri(&path).to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);
}
