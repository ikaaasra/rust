use crate::{
    model::{QueryOptions, ToDo, UpdateToDo, DB},
    response::{ToDoData, ToDoListResponse, ToDoSingleResponse},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

pub async fn health_handler() -> impl IntoResponse {
    return Json(
        serde_json::json!({"status":"success","message":"Build Simple CRUD(CREATE,READ,UPDATE,DELETE) API in Rust using Axum"}),
    );
}

// ----------------------------------------------------------------- CREATE_TODO
// pub async fn create_todo_handler()-> {}

// ----------------------------------------------------------------- GET_TODO
pub async fn get_todo_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let vec = db.lock().await;

    if let Some(todo) = vec.iter().find(|todo| todo.id == Some(id.to_owned())) {
        let json_response = ToDoSingleResponse {
            status: "success".to_string(),
            data: todo.clone(),
        };
        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Todo with ID: {} not found", id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

// ----------------------------------------------------------------- GET_TODOS
pub async fn get_todos_handler(
    options: Option<Query<QueryOptions>>,
    State(db): State<DB>,
) -> impl IntoResponse {
    let todos = db.lock().await;
    let Query(options) = options.unwrap_or_default();
    let limit = options.limit.unwrap_or(10);
    let offset = (options.page.unwrap_or(1) - 1) * limit;

    let todos: Vec<ToDo> = todos.clone().into_iter().skip(offset).take(limit).collect();

    Json(ToDoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        data: todos,
    })
}

// ----------------------------------------------------------------- UPDATE_TODO
// pub async fn update_todo_handler()-> {}

// ----------------------------------------------------------------- DELETE_TODO
// pub async fn delete_todo_handler()-> {}
