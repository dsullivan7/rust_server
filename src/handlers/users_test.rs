#[cfg(test)]

mod tests {
    use std::sync::Arc;

    use axum::{
        body::Body,
        http::{Method, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
    use serde_json::json;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::{
        handlers::{router, users::UserResponse, AppState},
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
        let body: UserResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(body.user_id, user_id);
        assert_eq!(body.first_name, Some("first_name".to_owned()));
        assert_eq!(body.last_name, Some("last_name".to_owned()));
        assert_eq!(body.created_at, user_db.created_at);
        assert_eq!(body.updated_at, user_db.updated_at);
    }

    #[tokio::test]
    async fn test_list_users() {
        let user_id_1 = Uuid::new_v4();
        let user_id_2 = Uuid::new_v4();

        let user_db_1: models::user::Model = models::user::Model {
            user_id: user_id_1.to_owned(),
            first_name: Some("first_name_1".to_owned()),
            last_name: Some("last_name_1".to_owned()),
            auth0_id: Some("auth0_id_1".to_owned()),
            created_at: chrono::Utc::now().into(),
            updated_at: chrono::Utc::now().into(),
        };

        let user_db_2: models::user::Model = models::user::Model {
            user_id: user_id_2.to_owned(),
            first_name: Some("first_name_2".to_owned()),
            last_name: Some("last_name_2".to_owned()),
            auth0_id: Some("auth0_id_2".to_owned()),
            created_at: chrono::Utc::now().into(),
            updated_at: chrono::Utc::now().into(),
        };

        let conn = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![user_db_1.clone(), user_db_2.clone()]])
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
                    .uri("/users")
                    .header(default_auth_header, default_auth_header_value)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Vec<UserResponse> = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.len(), 2);

        assert_eq!(body[0].user_id, user_id_1);
        assert_eq!(body[0].first_name, Some("first_name_1".to_owned()));
        assert_eq!(body[0].last_name, Some("last_name_1".to_owned()));
        assert_eq!(body[0].created_at, user_db_1.created_at);
        assert_eq!(body[0].updated_at, user_db_1.updated_at);

        assert_eq!(body[1].user_id, user_id_2);
        assert_eq!(body[1].first_name, Some("first_name_2".to_owned()));
        assert_eq!(body[1].last_name, Some("last_name_2".to_owned()));
        assert_eq!(body[1].created_at, user_db_2.created_at);
        assert_eq!(body[1].updated_at, user_db_2.updated_at);
    }

    #[tokio::test]
    async fn test_modify_user() {
        let user_id = Uuid::new_v4();

        let user_db: models::user::Model = models::user::Model {
            user_id: user_id.to_owned(),
            first_name: Some("first_name".to_owned()),
            last_name: Some("last_name".to_owned()),
            auth0_id: Some("auth0_id".to_owned()),
            created_at: chrono::Utc::now().into(),
            updated_at: chrono::Utc::now().into(),
        };

        let user_db_modified: models::user::Model = models::user::Model {
            user_id: user_id.to_owned(),
            first_name: Some("first_name_different".to_owned()),
            last_name: Some("last_name_different".to_owned()),
            auth0_id: Some("auth0_id".to_owned()),
            created_at: chrono::Utc::now().into(),
            updated_at: chrono::Utc::now().into(),
        };

        let conn = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![user_db.clone()]])
            .append_query_results(vec![vec![user_db_modified.clone()]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
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
                    .method(Method::PUT)
                    .uri(format!("/users/{}", user_db.user_id))
                    .header(default_auth_header, default_auth_header_value)
                    .header("content-type", "application/json")
                    .body(
                        json!(
                          {
                            "first_name": "first_name_different",
                            "last_name": "last_name_different"
                          }
                        )
                        .to_string(),
                    )
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: UserResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(body.user_id, user_id);
        assert_eq!(body.first_name, Some("first_name_different".to_owned()));
        assert_eq!(body.last_name, Some("last_name_different".to_owned()));
        assert_eq!(body.created_at, user_db_modified.created_at);
        assert_eq!(body.updated_at, user_db_modified.updated_at);
    }
}
