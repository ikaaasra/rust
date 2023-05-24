use crate::{
    model::{QueryOptions, ToDo, UpdateTodo, DB},
    response::{ToDoData, ToDoListResponse, ToDoSingleResponse},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

// pub async fn create_todo_handler()-> {}
// pub async fn get_todo_handler()-> {}
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
// pub async fn update_todo_handler()-> {}
// pub async fn delete_todo_handler()-> {}
