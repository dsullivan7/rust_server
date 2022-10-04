use actix_web::{http, test, App};
use sea_orm::{DatabaseBackend, MockDatabase};

use crate::test_utils;

#[cfg(test)]
#[tokio::test]
async fn test_get_point() {
    use super::*;

    let point_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let point_db: models::point::Model = models::point::Model {
        point_id: point_id.to_owned(),
        user_id: user_id.to_owned(),
        amount: 100,
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![point_db.clone()]])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    }
    .with_default_auth();

    let state = web::Data::new(test_state.into_app_state());
    let app = test::init_service(App::new().app_data(state).service(get_point)).await;

    let path = format!("/points/{}", point_id);
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
    //             r#"SELECT "point"."point_id", "point"."name", "point"."last_name", "point"."created_at", "point"."updated_at" FROM "points" LIMIT $1"#,
    //             vec![1u64.into()]
    //         ),
    //     ],
    // );

    let point_resp: models::point::Model = actix_web::test::read_body_json(resp).await;

    assert_eq!(point_resp.point_id, point_db.point_id);
    assert_eq!(point_resp.user_id, point_db.user_id);
    assert_eq!(point_resp.amount, point_db.amount);
    assert_eq!(point_resp.created_at, point_db.created_at);
    assert_eq!(point_resp.updated_at, point_db.updated_at);
}

#[cfg(test)]
#[tokio::test]
async fn test_list_points() {
    use super::*;

    let point_id_1 = Uuid::new_v4();
    let user_id_1 = Uuid::new_v4();

    let point_db: models::point::Model = models::point::Model {
        point_id: point_id_1.to_owned(),
        user_id: user_id_1.to_owned(),
        amount: 100,
        created_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        updated_at: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
    };

    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![point_db.clone()]])
        .into_connection();

    let test_state = test_utils::TestState {
        conn,
        ..Default::default()
    }
    .with_default_auth();

    let state = web::Data::new(test_state.into_app_state());

    let app = test::init_service(App::new().app_data(state).service(list_points)).await;

    let req = test::TestRequest::get()
        .uri("/points")
        .insert_header(test_utils::TestState::get_default_auth_header())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let points_resp: Vec<models::point::Model> = actix_web::test::read_body_json(resp).await;

    assert_eq!(points_resp[0].point_id, point_db.point_id);
    assert_eq!(points_resp[0].user_id, point_db.user_id);
    assert_eq!(points_resp[0].amount, point_db.amount);
    assert_eq!(points_resp[0].created_at, point_db.created_at);
    assert_eq!(points_resp[0].updated_at, point_db.updated_at);
}
