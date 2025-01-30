#[cfg(test)]
#[tokio::test]
async fn test_health() {
    use crate::authentication;
    use crate::authorization;
    use crate::handlers::router;
    use crate::handlers::{health::HealthResponse, AppState};
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use sea_orm::{DatabaseBackend, MockDatabase};
    use std::sync::Arc;
    use tower::ServiceExt;

    let conn = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

    let my_router = router(AppState {
        conn: Arc::new(conn),
        authentication: Arc::new(authentication::MockIAuthentication::new()),
        authorization: Arc::new(authorization::MockIAuthorization::new()),
    });

    let response = my_router
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: HealthResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(body.status, "healthy");
}
