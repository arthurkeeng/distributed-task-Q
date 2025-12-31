use std::collections::HashMap;

use common::{FieldType, PayloadField, Task, TaskPayloadSchema};
use async_trait::async_trait;
use serde_json::{Value, json};
use crate::handler::registry::{HandlerResult, TaskHandler};



pub struct EchoHandler ; 
#[async_trait]
impl TaskHandler for EchoHandler {
    async fn handle(&self , task : &Task)
     -> HandlerResult{
        let payload : Value = task.payload.clone();

        HandlerResult::ok(json!(
        {"echoed" : payload}
        ))
    }
}


impl EchoHandler{
    pub fn schema () -> TaskPayloadSchema {
        TaskPayloadSchema { task_type: "echo".to_string(), description: "Just a test handler to test worker".to_string(),fields: 
            HashMap::from([
                (
                    "echo".to_string(), 
                    PayloadField {
                        field_type : FieldType::String, 
                        required : true , 
                        description : Some("Just a test handler".to_string()), 
                        example : Some(json!("No worries , just testing things"))
                    }
                )
            ])
     }
    }

    

}