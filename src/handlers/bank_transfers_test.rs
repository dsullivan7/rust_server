use actix_web::{test, App};
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};

use crate::test_utils;

#[cfg(test)]
#[tokio::test]
async fn test_get_bank_transfer() {
    use super::*;

    let bank_transfer_id = Uuid::new_v4();
    let bank_account_id = Uuid::new_v4();

    let bank_transfer_db: models::bank_transfer::Model = models::bank_transfer::Model {
        bank_transfer_id: bank_transfer_id.to_owned(),
        bank_account_id: bank_account_id.to_owned(),
        amount: 100,
        dwolla_transfer_id: Some("dwolla_transfer_id".to_owned()),
        status: "status".to_owned(),
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![bank_transfer_db.clone()]])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    };

    let state = web::Data::new(test_state.into_app_state());
    let app = test::init_service(App::new().app_data(state).service(get_bank_transfer)).await;

    let path = format!("/bank-transfers/{}", bank_transfer_id);
    let req = test::TestRequest::get().uri(&path).to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let bank_transfer_resp: models::bank_transfer::Model =
        actix_web::test::read_body_json(resp).await;

    assert_eq!(
        bank_transfer_resp.bank_transfer_id,
        bank_transfer_db.bank_transfer_id
    );
    assert_eq!(
        bank_transfer_resp.bank_account_id,
        bank_transfer_db.bank_account_id
    );
    assert_eq!(
        bank_transfer_resp.dwolla_transfer_id,
        bank_transfer_db.dwolla_transfer_id
    );
    assert_eq!(bank_transfer_resp.amount, bank_transfer_db.amount);
    assert_eq!(bank_transfer_resp.status, bank_transfer_db.status);
    assert_eq!(bank_transfer_resp.created_at, bank_transfer_db.created_at);
    assert_eq!(bank_transfer_resp.updated_at, bank_transfer_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_list_bank_transfer() {
    use super::*;

    let bank_transfer_id = Uuid::new_v4();
    let bank_account_id = Uuid::new_v4();

    let bank_transfer_db: models::bank_transfer::Model = models::bank_transfer::Model {
        bank_transfer_id: bank_transfer_id.to_owned(),
        bank_account_id: bank_account_id.to_owned(),
        amount: 100,
        dwolla_transfer_id: Some("dwolla_transfer_id".to_owned()),
        status: "status".to_owned(),
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![bank_transfer_db.clone()]])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    };

    let state = web::Data::new(test_state.into_app_state());

    let app = test::init_service(App::new().app_data(state).service(list_bank_transfers)).await;

    let req = test::TestRequest::get().uri("/bank-transfers").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let bank_transfers_resp: Vec<models::bank_transfer::Model> =
        actix_web::test::read_body_json(resp).await;

    assert_eq!(
        bank_transfers_resp[0].bank_transfer_id,
        bank_transfer_db.bank_transfer_id
    );
    assert_eq!(
        bank_transfers_resp[0].bank_account_id,
        bank_transfer_db.bank_account_id
    );
    assert_eq!(
        bank_transfers_resp[0].dwolla_transfer_id,
        bank_transfer_db.dwolla_transfer_id
    );
    assert_eq!(bank_transfers_resp[0].amount, bank_transfer_db.amount);
    assert_eq!(bank_transfers_resp[0].status, bank_transfer_db.status);
    assert_eq!(
        bank_transfers_resp[0].created_at,
        bank_transfer_db.created_at
    );
    assert_eq!(
        bank_transfers_resp[0].updated_at,
        bank_transfer_db.updated_at
    );
}

#[cfg(test)]
#[tokio::test]
async fn test_create_bank_transfer() {
    use super::*;

    let bank_transfer_id = Uuid::new_v4();
    let bank_account_id = Uuid::new_v4();

    let bank_transfer_db: models::bank_transfer::Model = models::bank_transfer::Model {
        bank_transfer_id: bank_transfer_id.to_owned(),
        bank_account_id: bank_account_id.to_owned(),
        amount: 100,
        dwolla_transfer_id: Some("dwolla_transfer_id".to_owned()),
        status: "status".to_owned(),
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![bank_transfer_db.clone()]])
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

    let app = test::init_service(App::new().app_data(state).service(create_bank_transfer)).await;

    let body = serde_json::json!({
        "bank_account_id": bank_account_id.to_string(),
        "amount": 100,
    });

    let req = test::TestRequest::post()
        .set_json(&body)
        .uri("/bank-transfers")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::CREATED);

    let bank_transfer_resp: models::bank_transfer::Model =
        actix_web::test::read_body_json(resp).await;

    assert_eq!(
        bank_transfer_resp.bank_transfer_id,
        bank_transfer_db.bank_transfer_id
    );
    assert_eq!(
        bank_transfer_resp.bank_account_id,
        bank_transfer_db.bank_account_id
    );
    assert_eq!(
        bank_transfer_resp.dwolla_transfer_id,
        bank_transfer_db.dwolla_transfer_id
    );
    assert_eq!(bank_transfer_resp.amount, bank_transfer_db.amount);
    assert_eq!(bank_transfer_resp.status, bank_transfer_db.status);
    assert_eq!(bank_transfer_resp.created_at, bank_transfer_db.created_at);
    assert_eq!(bank_transfer_resp.updated_at, bank_transfer_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_modify_bank_transfer() {
    use super::*;

    let bank_transfer_id = Uuid::new_v4();
    let bank_account_id = Uuid::new_v4();

    let bank_transfer_db: models::bank_transfer::Model = models::bank_transfer::Model {
        bank_transfer_id: bank_transfer_id.to_owned(),
        bank_account_id: bank_account_id.to_owned(),
        amount: 100,
        dwolla_transfer_id: Some("dwolla_transfer_id".to_owned()),
        status: "status".to_owned(),
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let bank_transfer_id = Uuid::new_v4();

    let bank_transfer_db_modified: models::bank_transfer::Model = models::bank_transfer::Model {
        bank_transfer_id: bank_transfer_id.to_owned(),
        bank_account_id: bank_account_id.to_owned(),
        amount: 100,
        dwolla_transfer_id: Some("dwolla_transfer_id".to_owned()),
        status: "status".to_owned(),
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![bank_transfer_db.clone()]])
        .append_query_results(vec![vec![bank_transfer_db_modified.clone()]])
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

    let app = test::init_service(App::new().app_data(state).service(modify_bank_transfer)).await;

    let body = serde_json::json!({});

    let path = format!("/bank-transfers/{}", bank_transfer_id);
    let req = test::TestRequest::put()
        .set_json(&body)
        .uri(&path)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let bank_transfer_resp: models::bank_transfer::Model =
        actix_web::test::read_body_json(resp).await;

    assert_eq!(
        bank_transfer_resp.bank_transfer_id,
        bank_transfer_db_modified.bank_transfer_id
    );
    assert_eq!(
        bank_transfer_resp.bank_account_id,
        bank_transfer_db_modified.bank_account_id
    );
    assert_eq!(
        bank_transfer_resp.dwolla_transfer_id,
        bank_transfer_db_modified.dwolla_transfer_id
    );
    assert_eq!(bank_transfer_resp.amount, bank_transfer_db_modified.amount);
    assert_eq!(bank_transfer_resp.status, bank_transfer_db_modified.status);
    assert_eq!(
        bank_transfer_resp.created_at,
        bank_transfer_db_modified.created_at
    );
    assert_eq!(
        bank_transfer_resp.updated_at,
        bank_transfer_db_modified.updated_at
    );
}

#[cfg(test)]
#[tokio::test]
async fn test_delete_bank_transfer() {
    use super::*;

    let bank_transfer_id = Uuid::new_v4();

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

    let app = test::init_service(App::new().app_data(state).service(delete_bank_transfer)).await;

    let path = format!("/bank-transfers/{}", bank_transfer_id);
    let req = test::TestRequest::delete().uri(&path).to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);
}
