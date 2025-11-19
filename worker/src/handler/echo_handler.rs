use common::Task;
use async_trait::async_trait;
use serde_json::{Value, json};
use crate::handler::registry::{HandlerResult, TaskHandler};



pub struct EchoHandler ; 
#[async_trait]
impl TaskHandler for EchoHandler {
    async fn handle(&self , task : &Task)
     -> HandlerResult{
        let payload : Value = task.payload.clone();

        HandlerResult::success(Some(json!(
        {"echoed" : payload}
        )))
    }
}