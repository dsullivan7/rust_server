use actix_web::{http, test, App};

use mockall::predicate::*;

use crate::plaid;
use crate::test_utils;

#[cfg(test)]
#[tokio::test]
async fn test_create_token() {
    use super::*;

    let mut plaid_client = Box::new(plaid::MockIPlaidClient::new());

    plaid_client
        .expect_create_token()
        .with(eq(String::from("my_user_id")))
        .times(1)
        .return_const("my_token".to_string());

    let test_state = test_utils::TestState {
        plaid_client,
        ..Default::default()
    };

    let state = web::Data::new(test_state.into_app_state());

    let body = serde_json::json!({
        "user_id": "my_user_id",
    });

    let app = test::init_service(App::new().app_data(state).service(create_token)).await;

    let req = test::TestRequest::post()
        .set_json(&body)
        .uri("/plaid/token")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let token_resp: serde_json::Value = actix_web::test::read_body_json(resp).await;
    assert_eq!(token_resp["value"], "my_token");
}
