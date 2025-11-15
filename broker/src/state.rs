use std::{collections::HashMap, sync::Arc};
use common::{SubmitResultRequest, Task, TaskStatus};
use tokio::sync::Mutex;
use uuid::Uuid;


#[derive(Clone)]
pub struct BrokerState{
    pub tasks : Arc<Mutex<HashMap<Uuid, Task>>>
}

impl BrokerState{
    pub fn new() -> Self{
        Self { tasks: Arc::new(Mutex::new(HashMap::new())) }
    }

    pub async fn get_task(&self , id : Uuid) -> Option<Task>{
        let tasks = self.tasks.lock().await;
        tasks.get(&id).cloned()
    }

    pub async fn get_next_pending_task(&self) -> Option<Task>{
        let mut tasks = self.tasks.lock().await;

        let next = tasks.values_mut()
            .find(|t|t.status ==TaskStatus::Pending);

        if let Some(task) = next{
            task.mark_running();
            return Some(task.clone());
        }
        None
    }

    pub async fn update_task(&self , id : Uuid, status : TaskStatus)-> Option<Task>{
        let mut tasks = self.tasks.lock().await;

        if let Some(task) = tasks.get_mut(&id){
            task.status = status.clone();
            return Some(task.clone());
        }

        None
    }

    pub async fn complete_task(&self , id : Uuid , result : SubmitResultRequest) -> Option<Task>{
        let mut tasks = self.tasks.lock().await;

        let task = tasks.get_mut((&id))?;

        match (result.output , result.error){
            (Some(output) , None) => {
                task.mark_completed(Some(output));
            }
            (_ , Some(err)) =>{
                task.mark_failed(err);
            }
            _ => {
                task.mark_failed("Failed to process task".to_string());
            }
        }
        Some(task.clone())

    }
}
pub fn get(){}