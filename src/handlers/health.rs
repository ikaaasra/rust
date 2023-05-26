use crate::schema::GenericResponse;
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn health_handler() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    return Ok((
        StatusCode::OK,
        Json(serde_json::json!(GenericResponse {
            status: "success".to_string(),
            message: "Build Simple CRUD(CREATE,READ,UPDATE,DELETE) API in Rust using Axum"
                .to_string(),
        })),
    ));
}
