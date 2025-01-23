#[cfg(test)]

mod tests {
    use std::sync::Arc;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use sea_orm::{DatabaseBackend, MockDatabase};
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::{
        handlers::{router, users::UserRespose, AppState},
        models, test_utils,
    };

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

        let auth = test_utils::get_default_auth();

        let (default_auth_header, default_auth_header_value) =
            test_utils::get_default_auth_header();

        let router = router(AppState {
            conn: Arc::new(conn),
            authentication: Arc::from(auth),
        });

        let response = router
            .oneshot(
                Request::builder()
                    .uri(format!("/users/{}", user_db.user_id))
                    .header(default_auth_header, default_auth_header_value)
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
}
