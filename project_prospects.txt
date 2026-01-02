📦 Distributed Task Queue (Rust)
A high-performance, fault-tolerant distributed task queue written in Rust.
Inspired by systems like Celery, Temporal, Sidekiq, and BullMQ, but built from scratch with Rust’s speed, type-safety, and concurrency guarantees.
This project provides a full background-job execution system for modern applications, including:


Enqueuing tasks over HTTP
Persistent storage (SQLite for MVP)
Worker processes that execute jobs
Retries, backoff, and task locking
Horizontal scaling across multiple machines
Extensible design for custom tasks & workflows

🚀 Why This Project?
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

🧱 Architecture Overview
The system has four main components:
1. Broker
A standalone Rust server responsible for:


Storing tasks


Tracking task states (Pending, Running, Failed, Completed)


Assigning tasks to workers


Preventing duplicate execution


Retrying with backoff


Handling worker heartbeats


2. Worker
A background process running user-defined job functions.
Workers:


Poll the broker for pending tasks


Lock a task


Execute the associated function


Report results


Handle retries


Workers can run on multiple machines, enabling horizontal scaling.
3. Producer API / SDK
A Rust client library for sending tasks to the broker:
queue.enqueue("send_email", json!({"to": "user@example.com"}));

Other languages (Python, Node.js) can be added later.
4. Storage Layer
For MVP:


SQLite (via sqlx)


Later upgrades:


Postgres backend


RocksDB / LMDB


Custom WAL-based storage engine



🏁 MVP Features
The first working version will support:


Submit tasks via HTTP


Queue persistence


Pull tasks with locking


Worker registration


Heartbeats


Retries + backoff


Task results + metadata


Graceful worker shutdown


This MVP will already be equivalent to a simplified Celery/RQ system.

🔮 Planned Features
After MVP, the system can evolve with:
⚙ Advanced Task Features


Cron-style scheduled tasks


Delayed jobs


Dead-letter queues


Task dependencies (A → B → C)


Batch jobs


📡 Protocol Upgrades


gRPC support


Custom binary protocol over TCP


Zero-copy payload transfer


📊 Observability


Admin dashboard (Tauri or Web)


Metrics + Prometheus


Distributed tracing


⚡ Performance / Scaling


sharded queues


worker pools


persistent streaming


pluggable storage engines


🤖 AI / ML Integrations


long-running inference workers


embedding or Whisper jobs


vector DB integration



🧬 Project Structure
distributed-task-queue/
│
├── broker/      # Queue broker service
├── worker/      # Worker runtime
├── sdk/         # Client library for producers
└── common/      # Shared types, models, protocol

This Cargo workspace structure keeps the system modular and easy to scale.

🛠 Tech Stack
Rust crates:


tokio — async runtime


axum — HTTP server


reqwest — worker <-> broker communication


sqlx — SQLite persistence


serde — serialization


uuid — task IDs


chrono — time handling


tracing — logging



📥 Getting Started
Clone repo
git clone https://github.com/yourname/distributed-task-queue
cd distributed-task-queue

Run the broker
cd broker
cargo run

Run a worker
cd worker
cargo run

Enqueue a task (HTTP)
curl -X POST http://localhost:3000/enqueue \
  -H "Content-Type: application/json" \
  -d '{"task_type":"send_email","payload":{"to":"user@example.com"}}'


👥 Contributing
This project is built step-by-step as an educational + production-ready implementation.
Pull requests, issues, and discussions are welcome.
