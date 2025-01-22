use crate::authentication;
use crate::handlers::router;
use crate::handlers::users::UserRespose;
use crate::handlers::AppState;
use crate::models;

use std::future;
use std::sync::Arc;
use tower::ServiceExt;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use mockall::predicate::eq;
use sea_orm::DatabaseBackend;
use sea_orm::MockDatabase;
use uuid::Uuid;

const DEFAULT_AUTH0_ID: &str = "default_auth0_id";
const DEFAULT_AUTH0_TOKEN: &str = "default_auth0_token";

#[cfg(test)]
#[tokio::test]
async fn test_get_user() {
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

    let mut auth = authentication::MockIAuthentication::new();
    auth.expect_validate_token()
        .with(eq(String::from(DEFAULT_AUTH0_TOKEN)))
        .times(1)
        .returning(|_| {
            Box::pin(future::ready(Ok(authentication::Claims {
                sub: DEFAULT_AUTH0_ID.to_string(),
            })))
        });

    let my_router = router(AppState {
        conn: Arc::new(conn),
        authentication: Arc::new(auth),
    });

    let response = my_router
        .oneshot(
            Request::builder()
                .uri(format!("/users/{}", user_db.user_id))
                .header("Authorization", format!("Bearer {}", DEFAULT_AUTH0_TOKEN))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: UserRespose = serde_json::from_slice(&body).unwrap();
    assert_eq!(body.user_id, user_id);
    assert_eq!(body.first_name, Some("first_name".to_owned()));
    assert_eq!(body.last_name, Some("last_name".to_owned()));
    assert_eq!(body.created_at, user_db.created_at);
    assert_eq!(body.updated_at, user_db.updated_at);
}
