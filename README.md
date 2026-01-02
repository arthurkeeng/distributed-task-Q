# Conduit Distributed Task Queue API

## Overview

A high-performance, fault-tolerant distributed task queue written in Rust.
Inspired by systems like Celery, Temporal, Sidekiq, and BullMQ, but built from scratch with Rust’s speed, type-safety, and concurrency guarantees.
This project provides a full background-job execution system for modern applications, including:

Enqueuing tasks over HTTP
Persistent storage (SQLite for MVP)
Worker processes that execute jobs
Retries, backoff, and task locking
Horizontal scaling across multiple machines
Extensible design for custom tasks & workflows

Why This Project?
Distributed task queues are the backbone of large-scale systems. They power:

Email sending
Video/image processing
Payment workflows
Search indexing
AI model inference
Data pipelines
Cron scheduling
Background automation
Rust is an excellent fit for high-reliability queue systems:
Memory-safe (no segmentation faults, no data races)
Fast concurrency via tokio
Predictable performance
Low resource usage — ideal for many workers
Great for distributed systems
This project demonstrates real-world system design using modern Rust.

## Overview

## Features
- **Scalable Broker**: Asynchronous task management using Axum and Tokio.
- **Robust Workers**: Extensible worker registry with built-in handlers for task handling. Handlers currently set are echo tasks to check that setup was correct and image validation hanlder. More handlers will be added subsequently
- **Cross-Platform SDKs**: Native support for both Rust and TypeScript/JavaScript environments.
- **Schema Validation**: Dynamic payload schema registration and retrieval for type-safe task processing.
- **Real-time Polling**: Efficient worker-to-broker polling mechanism with configurable intervals.

## Getting Started

### Installation

**1. Clone the Repository**
```bash
git clone https://github.com/arthurkeeng/distributed-task-Q
cd distributed-task-queue
```

**2. Setup the Broker (Rust)**
```bash
cd broker
cargo build --release
cargo run
```

**3. Setup the Worker (Rust)**
```bash
cd ../worker
# Configure .env file first
cargo run
```


### Environment Variables
The worker requires the following environment variables to communicate with the broker. Create a `.env` file in the `/worker` directory:

| Variable | Example | Description |
| :--- | :--- | :--- |
| `BROKER_URL` | `http://localhost:8080` | The URL where the broker service is running. |
| `WORKER_NAME` | `image-processor-01` | Unique identifier for the worker instance. |
| `POLL_INTERVAL_MS` | `500` | Frequency in milliseconds to poll the broker for new tasks. |

## API Documentation

### Base URL
`http://localhost:8080/task`

### Endpoints

#### POST /
**Request**:
_Body Example:_
```json
{
  "task_type": "validate_image",
  "payload": {
    "image": "base64_encoded_string_here"
  }
}
```

**Response**:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "task_type": "validate_image",
  "payload": {
    "image": "base64_encoded_string_here"
  },
  "status": "Pending",
  "result": null,
  "created_at": "2023-10-27T10:00:00Z",
  "started_at": null,
  "completed_at": null
}
```

**Errors**:
- 400: Malformed JSON payload.

#### GET /:id
**Request**:
_Path Parameter:_ `id` (UUID)

**Response**:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "task_type": "validate_image",
  "status": "Completed",
  "result": {
    "output": {
      "valid": true,
      "width": 1024,
      "height": 768
    },
    "error": null
  }
}
```

**Errors**:
- 404: Task ID not found.

#### GET /next
**Request**:
_Used by workers to fetch the next pending task._

**Response**:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "task_type": "validate_image",
  "payload": { ... },
  "status": "Running"
}
```

**Errors**:
- 204: No tasks currently in queue.

#### POST /:id/result
**Request**:
_Body Example:_
```json
{
  "output": {
    "valid": true,
    "width": 800
  },
  "error": null
}
```

**Response**:
```json
{
  "status": "Completed"
}
```

**Errors**:
- 404: Task ID not found.

#### GET /types
**Request**:
_Fetches all registered task types._

**Response**:
```json
["echo", "validate_image"]
```

#### GET /types/:task_type/schema
**Request**:
_Path Parameter:_ `task_type` (String)

**Response**:
```json
{
  "task_type": "validate_image",
  "description": "Validate that a base64 image is real and large enough",
  "fields": {
    "image": {
      "field_type": "string",
      "required": true,
      "description": "Base 64 Encoded image"
    }
  }
}
```

## Technologies Used

| Technology | Purpose |
| :--- | :--- |
| [Rust](https://www.rust-lang.org/) | Core Backend & Worker Logic |
| [Axum](https://github.com/tokio-rs/axum) | Web Framework for Broker |
| [Tokio](https://tokio.rs/) | Asynchronous Runtime |
| [Serde](https://serde.rs/) | Serialization/Deserialization |


## Contributing
- 📂 **Fork the Repository**: Create your feature branch.
- 🛠️ **Quality Control**: Ensure Rust code is formatted with `cargo fmt`.
- 🧪 **Testing**: Add tests in the `common` or `sdk` crates for new logic.
- 📝 **Pull Requests**: Provide detailed descriptions of changes.

## Author Info
- **Github**: [arthurkeeng](https://github.com/arthurkeeng)
- **LinkedIn**: [linkedin](https://linkedin.com/in/arthur-chima)
- **Portfolio**: [portfolio](https://omeenee.vercel.app)

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

![Tokio](https://img.shields.io/badge/Tokio-async-blue?style=for-the-badge)

