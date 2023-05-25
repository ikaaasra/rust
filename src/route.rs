use crate::{
    handler::{
        create_todo_handler, delete_todo_handler, get_todo_handler, get_todos_handler,
        health_handler, update_todo_handler,
    },
    AppState,
};
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn router(app_state: Arc<AppState>) -> Router {
    return Router::new()
        .route("/api/health", get(health_handler))
        .route(
            "/api/todos",
            get(get_todos_handler).post(create_todo_handler),
        )
        .route(
            "/api/todos/:id",
            get(get_todo_handler)
                .delete(delete_todo_handler)
                .patch(update_todo_handler),
        )
        .with_state(app_state);
}
