use crate::{
    model::ToDoModel,
    schema::{
        CreateToDo, FilterOptions, GenericResponse, ToDoListResponse, ToDoSingleResponse,
        UpdateToDo,
    },
    AppState,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;

// ----------------------------------------------------------------- CREATE_TODO
pub async fn create_todo_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateToDo>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query_as!(
        ToDoModel,
        "INSERT INTO todos (title,content) VALUES ($1,$2) RETURNING *",
        body.title.to_string(),
        body.content.to_string(),
    )
    .fetch_one(&data.db)
    .await;

    match query {
        Ok(todo) => {
            let json_response = serde_json::json!(ToDoSingleResponse {
                status: "".to_string(),
                data: todo,
            });
            return Ok((StatusCode::CREATED, Json(json_response)));
        }
        Err(err) => {
            if err
                .to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!(GenericResponse {
                    status: "fail".to_string(),
                    message: "ToDo with that title already exists".to_string(),
                });
                return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
            }

            let error_response = serde_json::json!(GenericResponse {
                status: "error".to_string(),
                message: format!("{:?}", err),
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    }
}

// ----------------------------------------------------------------- GET_TODO
pub async fn get_todo_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query_as!(ToDoModel, "SELECT * FROM todos WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    match query {
        Ok(todo) => {
            let json_response = serde_json::json!(ToDoSingleResponse {
                status: "".to_string(),
                data: todo,
            });
            return Ok((StatusCode::CREATED, Json(json_response)));
        }
        Err(_) => {
            let error_response = serde_json::json!(GenericResponse {
                status: "".to_string(),
                message: format!("ToDo with ID: {} not found", id)
            });

            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

// ----------------------------------------------------------------- GET_TODOS
pub async fn get_todos_handler(
    options: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(options) = options.unwrap_or_default();

    let limit = options.limit.unwrap_or(10);
    let offset = (options.page.unwrap_or(1) - 1) * limit;

    let query = sqlx::query_as!(
        ToDoModel,
        "SELECT * FROM todos ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query.is_err() {
        let error_response = serde_json::json!(GenericResponse {
            status: "fail".to_string(),
            message: "Something bad happened while fetching all todo items".to_string(),
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let todos = query.unwrap();

    let json_response = serde_json::json!(ToDoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        data: todos,
    });

    return Ok((StatusCode::OK, Json(json_response)));
}

// ----------------------------------------------------------------- UPDATE_TODO
pub async fn update_todo_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateToDo>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query_as!(ToDoModel, "SELECT * FROM todos WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    if query.is_err() {
        let error_response = serde_json::json!(GenericResponse {
            status: "fail".to_string(),
            message: format!("ToDo with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let todo = query.unwrap();

    let query = sqlx::query_as!(
        ToDoModel,
        "UPDATE todos SET title = $1, content = $2, complete = $3, updated_at = $4 WHERE id = $5 RETURNING *",
        body.title.to_owned().unwrap_or(todo.title),
        body.content.to_owned().unwrap_or(todo.content),
        body.complete.unwrap_or(todo.complete.unwrap()),
        now,
        id
    )
    .fetch_one(&data.db)
    .await;

    match query {
        Ok(todo) => {
            let json_response = serde_json::json!(ToDoSingleResponse {
                status: "success".to_string(),
                data: todo
            });

            return Ok((StatusCode::OK, Json(json_response)));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(GenericResponse {
                    status: "error".to_string(),
                    message: format!("{:?}", err)
                })),
            ));
        }
    }
}

// ----------------------------------------------------------------- DELETE_TODO
pub async fn delete_todo_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if query == 0 {
        let error_response = serde_json::json!(GenericResponse {
            status: "fail".to_string(),
            message: format!("ToDo with ID: {} not found", id)
        });

        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    return Ok((StatusCode::NO_CONTENT, Json({})));
}
