use crate::handler::{config::WorkerConfig, echo_handler::EchoHandler, registry::HandlerRegistry, validate_image_handler::ValidateImageHandler, worker::Worker};


pub mod handler;

#[tokio::main]
async fn main() {

    let cfg = WorkerConfig::from_env();

    let mut registry = HandlerRegistry::new();
    registry.register_handler("echo", EchoHandler);
    registry.register_handler("validate_image", ValidateImageHandler);

    let worker = Worker::new(cfg, registry);
    worker.run().await;
}