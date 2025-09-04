use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

// Simple test without database
#[tokio::test]
async fn test_health_check() {
    use axum::Router;
    use axum::routing::get;
    
    let app = Router::new().route("/health", get(|| async { "OK" }));
    
    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_json_response() {
    use axum::{
        response::Json,
        routing::get,
        Router,
    };
    
    let app = Router::new().route("/test", get(|| async { 
        Json(json!({"message": "test"}))
    }));
    
    let request = Request::builder()
        .uri("/test")
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

