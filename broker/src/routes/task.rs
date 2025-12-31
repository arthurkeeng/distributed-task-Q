use axum::{Json, body, extract::{Path , State}, http::StatusCode};

use common::{ SubmitResultRequest, SubmitResultResponse, Task, TaskPayloadSchema, TaskResult, TaskStatus};
use serde_json::Value;
use uuid::Uuid;

use crate::AppState;

// To create a new task , you must pass the payload this way 

// {
//     "task_type" :"name of task type", 
//     "payload" : Value
// }

pub async fn create_task(
    State(state) : State<AppState>,
    Json(payload) : Json<Value>
) -> Json<Task>{


    let task_type = payload.get("task_type")
        .and_then(|v| v.as_str())
        .unwrap_or("default_task");

    let task_payload = payload.get("payload")
        .cloned()
        .unwrap_or(Value::Null);

    let task = Task::new(task_type, task_payload);
    state.tasks.lock().await.insert(task.id, task.clone());
    state.queue.lock().await.push_front(task.id);
    Json(task)

}
pub async fn get_task(
    State(state) : State<AppState>,
    Path(id) : Path<Uuid>
) -> Result<Json<Task> , StatusCode>{
    let task = state.tasks.lock().await;

     match task.get(&id){
        Some(t) => Ok(Json(t.clone())), 
        None => Err(StatusCode::NOT_FOUND)
    }

}
pub async fn get_next_task(
    State(state) : State<AppState>
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
    State(state) : State<AppState>, 
    Path(id) : Path<Uuid>, 
    Json(body) : Json<SubmitResultRequest>
) -> Json<SubmitResultResponse>{
    let mut tasks = state.tasks.lock().await;

    let Some(task) = tasks.get_mut(&id)
    else {
        return Json(SubmitResultResponse { status: TaskStatus::Failed })
    };

    if let Some(err) = &body.error{
        task.mark_failed(err.clone());
    }
    else{
        task.mark_completed(body.output.clone());
    }
    Json(SubmitResultResponse { status: task.status.clone() })
}

pub async fn list_task_types(State(state) : State<AppState>, )
-> Json<Vec<String>>
{
    let schemas = state.payload_schemas.lock().await;

    Json(schemas.keys().cloned().collect())
}
pub async fn get_payload_schema(State(state) : State<AppState>,
    Path(task_type ): Path<String>
) ->Result<Json<TaskPayloadSchema>, StatusCode>{
    let schemas = state.payload_schemas.lock().await;

    schemas.get(&task_type).cloned().map(Json).ok_or(StatusCode::NOT_FOUND)
}


pub async fn set_payload_schema(State(state) : State<AppState> , 
     Json(schema) : Json<TaskPayloadSchema>
    ) ->Json<TaskPayloadSchema> {
        let mut payload_schemas = state.payload_schemas.lock().await;

        payload_schemas.insert(schema.task_type.clone() , schema.clone());
        Json(schema)

    }