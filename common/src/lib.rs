use serde::{Serialize , Deserialize};
use serde_json::Value;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive( Debug, Clone , Deserialize , Serialize)]
pub struct Task {
    pub id : Uuid , 
    pub task_type : String, 
    pub payload : serde_json::Value ,
    pub status : TaskStatus ,
    pub result : Option<TaskResult>,

       /// Timestamps
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone , Deserialize , PartialEq , Eq , Serialize)]
pub enum TaskStatus {
    Pending , 
    Running , 
    Completed,
    Failed
}
#[derive(   Debug, Clone , Deserialize , Serialize)]
pub struct TaskResult{
    // optional output data 
    pub output : Option<serde_json::Value>, 
    pub error : Option<String>
}


impl Task{
    pub fn new(task_type : &str , payload : serde_json::Value) -> Self{
        Self { id: Uuid::new_v4(),
             task_type : task_type.to_string(), 
             payload,
              status:TaskStatus::Pending, 
              result: None, 
              created_at: Utc::now(),
               started_at: Utc::now().into(), 
               completed_at: Utc::now().into()
             }
    }

    pub fn mark_running(&mut self){
        self.status = TaskStatus::Running;

        self.started_at =Some(Utc::now())
    }
    pub fn mark_completed(&mut self , output : Option<serde_json::Value>){
        self.status = TaskStatus::Completed;
        self.result = Some(TaskResult { output, error:None });
        self.completed_at =Some(Utc::now())
    }
    pub fn mark_failed(&mut self , error_msg : String){
        self.status = TaskStatus::Failed;
        self.result = Some(TaskResult { output: None, error: Some(error_msg) });
        self.completed_at =Some(Utc::now())
    }

}


#[derive(Debug, Clone, Serialize , Deserialize)]
pub struct CreateTaskResponse{
    pub id :Uuid , 
    pub status : TaskStatus
}

#[derive(Debug, Clone, Serialize , Deserialize)]
pub struct GetTaskResponse{
    pub task : Task
}
#[derive(Debug, Clone, Serialize , Deserialize)]
pub struct SubmitResultRequest{
    pub output : Option<Value>,
    pub error : Option<String>
}

#[derive(Debug, Clone, Serialize , Deserialize)]
pub struct SubmitResultResponse{
    pub status : TaskStatus
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let new_task = Task::new("change image", serde_json::json!({"data" : "Hello from value"}));

        assert_eq!(new_task.task_type , "change image");
    }
}
