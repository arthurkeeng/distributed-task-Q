
use axum::{
    routing::{post, get}, 
    Json , Router , extract::Path, http::StatusCode
};
use serde_json::Value;
use std::{collections::{HashMap , VecDeque} , sync::Arc};
use uuid::{self, Uuid}; 
use chrono::Utc;
use common::{Task, TaskResult, TaskStatus};
use tokio::sync::Mutex as AsyncMutex;
mod state;
#[derive(Clone)]
struct  AppState{
    tasks : Arc<AsyncMutex<HashMap<Uuid , Task>>>,
    queue : Arc<AsyncMutex<VecDeque<Uuid>>>
}

#[tokio::main]
async fn main(){
    let state = AppState{
        tasks: Arc::new(AsyncMutex::new(HashMap::new())),
        queue :Arc::new(AsyncMutex::new(VecDeque::new()))
    };

    let app = Router::new()
        .route("/task", post(create_task))
        .route("/task/:id", get(get_task))
        .route("/task/next", get(get_next_task))
        .route("/task/:id/result", post(submit_result))
        .with_state(state);

    println!("Running at port 8080");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener , app).await.unwrap();
}

async fn create_task(
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
async fn get_task(
    axum::extract::State(state) : axum::extract::State<AppState>,
    axum::extract::Path(id) : axum::extract::Path<Uuid>
) -> Result<Json<Task> , StatusCode>{
    let task = state.tasks.lock().await;

     match task.get(&id){
        Some(t) => Ok(Json(t.clone())), 
        None => Err(StatusCode::NOT_FOUND)
    }

}
async fn get_next_task(
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
async fn submit_result(
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