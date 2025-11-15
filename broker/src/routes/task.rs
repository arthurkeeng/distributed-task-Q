use axum::{Json, extract::Path, http::StatusCode};
use common::{Task, TaskResult};
use serde_json::Value;
use uuid::Uuid;

use crate::AppState;


pub async fn create_task(
    axum::extract::State(state) : axum::extract::State<AppState>,
    Json(payload) : Json<Value>
) -> Json<Task>{
    // the payload will be something like 
    // {"task_type" : "", "payload" : ""}

    let task_type = payload.get("task_type")
        .and_then(|v| v.as_str())
        .unwrap_or("default_task");

    let task_payload = payload.get("payload")
        .cloned().unwrap_or(Value::Null);

    let task = Task::new(task_type, task_payload);
    state.tasks.lock().await.insert(task.id, task.clone());
    state.queue.lock().await.push_front(task.id);
    Json(task)

}
pub async fn get_task(
    axum::extract::State(state) : axum::extract::State<AppState>,
    axum::extract::Path(id) : axum::extract::Path<Uuid>
) -> Result<Json<Task> , StatusCode>{
    let task = state.tasks.lock().await;

     match task.get(&id){
        Some(t) => Ok(Json(t.clone())), 
        None => Err(StatusCode::NOT_FOUND)
    }

}
pub async fn get_next_task(
    axum::extract::State(state) : axum::extract::State<AppState>
) -> Result<Json<Task> , StatusCode>{

    let mut queue = state.queue.lock().await;

    if let Some(task_id) = queue.pop_front(){
        let mut tasks = state.tasks.lock().await;

        if let Some(task) = tasks.get_mut(&task_id){
            task.mark_running();
            return Ok(Json(task.clone()));
        }
    }
    Err(StatusCode::NO_CONTENT)
}
pub async fn submit_result(
    axum::extract::State(state) : axum::extract::State<AppState>, 
    Path(id) : Path<Uuid>, 
    Json(result) : Json<TaskResult>
) -> Result<StatusCode, StatusCode>{
    let mut tasks = state.tasks.lock().await;

    if let Some(task) = tasks.get_mut(&id){
        if result.error.is_some(){
            task.mark_failed(result.error.unwrap());
        }
        else {
            task.mark_completed(result.output);
            
        }
        return Ok(StatusCode::OK);
    }
    Err(StatusCode::NOT_FOUND)
}