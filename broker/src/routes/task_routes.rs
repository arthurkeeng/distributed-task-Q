use axum::{Router, routing::{post, get}};

use crate::{AppState, routes::task::{create_task, get_next_task, get_task, submit_result}};


pub fn task_routes () -> Router<AppState>{
    Router::new()
        .route("/", post(create_task))
        .route("/:id", get(get_task))
        .route("/next", get(get_next_task))
        .route("/:id/result", post(submit_result))
}