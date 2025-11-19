
use std::env;

pub struct WorkerConfig{
    pub broker_url :String, 
    pub worker_name : String , 

    
    pub poll_interval_ms : u64
}


impl WorkerConfig{
    pub fn from_env() -> Self{
        dotenvy::dotenv().ok();

        Self { broker_url:
            std::env::var("BROKER_URL")
            .unwrap_or("http://localhost:3000".into())
            , worker_name: env::var("WORKER_NAME")
            .unwrap_or("default-worker".into())
            ,
             poll_interval_ms: env::var("POLL_INTERVAL_MS").ok()
             .and_then(|v| v.parse().ok())
             .unwrap_or(1000),
            }
    }
}