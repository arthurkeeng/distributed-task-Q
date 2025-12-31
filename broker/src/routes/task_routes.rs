use axum::{Router, routing::{post, get}};

use crate::{AppState, routes::task::{create_task, get_next_task, get_task, submit_result , get_payload_schema, set_payload_schema, list_task_types}};


pub fn task_routes () -> Router<AppState>{
    Router::new()
        .route("/", post(create_task))
        .route("/:id", get(get_task))
        .route("/next", get(get_next_task))
        .route("/:id/result", post(submit_result))
        .route("/types" , get(list_task_types))
        .route("/types/:task_type/schema" , get(get_payload_schema))
        .route("/set_schema" , post(set_payload_schema))

}