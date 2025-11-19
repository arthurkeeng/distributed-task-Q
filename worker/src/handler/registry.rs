use std::collections::HashMap;

use serde_json::Value;
use async_trait::async_trait;
use common::Task;
pub struct HandlerResult{
    pub output : Option<Value>, 
    pub error : Option<String>
}

impl HandlerResult{
    pub fn success(output : Option<Value>) ->Self{
        Self{
            output , error : None
        }
    }

    pub fn failure (msg: String) -> Self{
        Self{
            output : None , error: Some(msg)
        }
    }
}

#[async_trait]
pub trait TaskHandler : Send + Sync {
        async fn handle(&self , task : &Task) -> HandlerResult ;
}
#[derive(Default)]
pub struct HandlerRegistry{
    pub handlers : HashMap<String , Box<dyn TaskHandler>>
}

impl HandlerRegistry{
    pub fn new() -> Self{
        Self { handlers: HashMap::new() }
    }

    pub fn register_handler<T> (&mut self , task_type : &str , handler : T)
    where  T : TaskHandler + 'static{
        self.handlers.insert(task_type.to_string(), Box::new(handler));
    }

    pub fn get(&self , task_type : &str) -> Option<&Box<dyn TaskHandler>>{
        self.handlers.get(task_type)
    }
}
