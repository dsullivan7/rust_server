use actix_web::{test, App};

#[actix_web::test]
async fn test_init_service() {
    use super::*;
    let app = test::init_service(App::new().service(get_user)).await;

    let req = test::TestRequest::get().uri("/users/blah").to_request();
    let resp = test::call_service(&app, req).await;
    let body = test::read_body(resp).await;
    assert_eq!(body, actix_web::web::Bytes::from("get user blah"));
}
