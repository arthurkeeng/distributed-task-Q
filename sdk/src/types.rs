use serde::{Serialize , Deserialize};
use serde_json::Value;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug , Clone , Serialize , Deserialize , PartialEq , Eq)]
pub enum TaskStatus {
    Pending , 
    Running , 
    Completed , 
    Failed 
}

#[derive(Debug , Clone , Serialize , Deserialize)]
pub struct TaskResult{
    pub output : Option<String>, 
    pub error : Option<String>
}

#[derive(Debug , Clone , Serialize , Deserialize)]
pub struct Task {
    pub id : Uuid,
    pub task_type: String,
    pub payload: Value,
    pub status: TaskStatus,
    pub result: Option<TaskResult>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub task_type : String , 
    pub payload : Value
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitResultRequest {
    pub output: Option<Value>,
    pub error: Option<String>,
}
