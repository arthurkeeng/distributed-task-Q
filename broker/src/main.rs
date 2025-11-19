
use axum::{ Router};
use std::{collections::{HashMap , VecDeque} , sync::Arc};
use uuid::{self, Uuid}; 

use common::{Task};
use tokio::sync::Mutex as AsyncMutex;
// mod state;
mod routes;


use crate::routes::task_routes::task_routes;

#[derive(Clone)]
struct  AppState{
    tasks : Arc<AsyncMutex<HashMap<Uuid , Task>>>,
    queue : Arc<AsyncMutex<VecDeque<Uuid>>>
}

#[tokio::main]
async fn main(){
    let state = AppState{
        tasks: Arc::new(AsyncMutex::new(HashMap::new())),
        queue :Arc::new(AsyncMutex::new(VecDeque::new()))
    };

    let app = Router::new()
        .nest("/task", task_routes())
        .with_state(state);

    println!("Running at port 8080");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener , app).await.unwrap();
}
