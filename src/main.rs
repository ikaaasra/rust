use axum::{response::IntoResponse, routing::get, Json, Router};

async fn health_handler() -> impl IntoResponse {
    return Json(
        serde_json::json!({"status":"success","message":"Build Simple CRUD(CREATE,READ,UPDATE,DELETE) API in Rust using Axum"}),
    );
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/health", get(health_handler));

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
