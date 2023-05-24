use crate::{
    handler::{create_todo_handler, get_todo_handler, get_todos_handler, health_handler},
    model,
};
use axum::{routing::get, Router};

pub fn router() -> Router {
    let db = model::todo_db();

    return Router::new()
        .route("/api/health", get(health_handler))
        .route(
            "/api/todos",
            get(get_todos_handler).post(create_todo_handler),
        )
        .route("/api/todos/:id", get(get_todo_handler))
        .with_state(db);
}
