use crate::{
    auth::auth,
    handlers::{
        auth::{get_me_handler, logout_handler, signin_handler, signup_handler},
        health::health_handler,
        todo::{
            create_todo_handler, delete_todo_handler, get_todo_handler, get_todos_handler,
            update_todo_handler,
        },
    },
    AppState,
};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
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
        .route("/auth/signin", post(signin_handler))
        .route("/auth/signup", post(signup_handler))
        .route(
            "/api/auth/logout",
            get(logout_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/api/users/me",
            get(get_me_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .with_state(app_state);
}
