use reqwest::Client as HttpClient;
use serde_json::Value;
use uuid::Uuid;



use crate::types::*;
pub struct TaskQueueClient{
    base_url :String , 
    http : HttpClient
}

impl TaskQueueClient{
    pub fn new(base_url : impl Into<String>) -> Self{
        Self { base_url : base_url.into(), http: HttpClient::new() }
    }

    fn url (&self , path : &str) -> String {
        format!("{}/{}" , self.base_url , path)
    }

    pub async fn create_task(&self , task_type : &str , payload : Value ) -> Result<Task , reqwest::Error>{
        let body = CreateTaskRequest{
            task_type : task_type.to_string(),
            payload
        };

        let res = self.http
                .post(self.url("task"))
                .json(&body)
                .send()
                .await?;
            res.json::<Task>().await
    }

    pub async fn get_task (&self , id : &Uuid) -> Result<Task , reqwest::Error>{
        let res = self.http
            .get(self.url(&format!("task/{}" , id)))
            .send()
            .await?;

        res.json::<Task>().await
    }
}