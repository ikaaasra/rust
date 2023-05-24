use crate::{
    handler::{get_todos_handler, health_handler},
    model,
};
use axum::{routing::get, Router};

pub fn router() -> Router {
    let db = model::todo_db();

    return Router::new()
        .route("/api/todos", get(get_todos_handler))
        .route("/api/health", get(health_handler))
        .with_state(db);
}
