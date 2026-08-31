use axum::{routing::post, Router, response::IntoResponse, http::StatusCode};
use axum::http::Request;
use hyper::Body;
use tower::ServiceExt;

async fn create_user() -> impl IntoResponse {
    (StatusCode::CREATED, "{"id": 1}")
}

#[tokio::test]
async fn test_create_user_returns_201() {
    let app = Router::new().route("/api/v1/users", post(create_user));
    
    let response = app
        .oneshot(Request::builder()
            .method("POST")
            .uri("/api/v1/users")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"email":"a@x.com","name":"Ada"}"#))
            .unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}
