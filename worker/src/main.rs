use crate::handler::{config::WorkerConfig, echo_handler::EchoHandler, registry::HandlerRegistry, worker::Worker};


pub mod handler;

#[tokio::main]
async fn main() {

    let cfg = WorkerConfig::from_env();

    let mut registry = HandlerRegistry::new();
    registry.register_handler("echo", EchoHandler);

    let worker = Worker::new(cfg, registry);
    worker.run().await;
}
