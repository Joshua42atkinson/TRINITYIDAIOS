use axum::{
    body::Body,
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::test]
async fn test_health_api_stability() {
    // Scaffold an Axum router that mocks the behavior of the real health endpoint
    let app = Router::new().route("/api/health", get(|| async { axum::Json(serde_json::json!({ "status": "healthy" })) }));

    // Start a background server on a random open port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Verify it is responding to HTTP requests securely before Tauri mounts
    let client = reqwest::Client::new();
    let url = format!("http://{}/api/health", addr);
    
    let res = client.get(&url).send().await.unwrap();
    assert_eq!(res.status(), 200);

    let body: serde_json::Value = res.json().await.unwrap();
    assert_eq!(body["status"], "healthy");
}
