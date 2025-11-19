use std::error;

use common::{SubmitResultRequest, Task};
use tokio::time::{sleep , Duration};

use crate::handler::{config::WorkerConfig, registry::{HandlerRegistry, HandlerResult}};
use reqwest::Client;

pub struct Worker{
    config  : WorkerConfig, 
    client : Client,
    registry : HandlerRegistry
}

impl Worker {
    pub fn new(config : WorkerConfig , registry : HandlerRegistry)-> Self{
        Self { config, client:Client::new(), registry }

    }

    pub async fn run(&self) {
        println!("Worker {} started . Polling broker at {}" , 
            self.config.worker_name , self.config.broker_url
     );
     

     loop{
        if let Err(e) = self.process_once().await{
            eprintln!("Worker error : {:?}" ,e);
        }
        sleep(Duration::from_millis(self.config.poll_interval_ms)).await;
     }
    }

    async fn process_once(&self) -> Result<() , Box<dyn error::Error>>{
        let next_url = format!("{}/task/next", self.config.broker_url);
        let res = self.client.get(&next_url).send().await?;

        if res.status() == reqwest::StatusCode::NO_CONTENT{
            return Ok(());
        }

        if !res.status().is_success(){
            eprintln!("Bad response from broker :{}" , res.status());
            return Ok(())
        }

        let task : Task = res.json().await?;

        println!("Received task {}" , task.id);

        let handler = match self.registry.handlers.get(&task.task_type){
            Some(h) => h, 
            None => {
                eprintln!("No handler registered for task type '{}'", task.task_type);
                return Ok(());
            }
        };

        let result : HandlerResult = handler.handle(&task.clone()).await;

        let submit_url = format!("{}/task/{}/result", self.config.broker_url , task.id);

        let req_body = SubmitResultRequest{
            output : result.output, 
            error : result.error
        };

        let _ = self.client
            .post(&submit_url)
            .json(&req_body)
            .send()
            .await?;
        println!("📤 Sent result for {}", task.id);
        Ok(())
    }
}