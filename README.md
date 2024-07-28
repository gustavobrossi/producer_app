
# Producer App

## Overview
The `producer_app` is a simple Kafka producer written in Rust. It sends a message to a Kafka topic.

## Requirements
- Rust
- Docker & Docker Compose (optional to run Kafka Services)

## Setup Instructions

1. **Clone the Repository**

   ```sh
   git clone git@github.com:gustavobrossi/producer_app.git
   cd producer_app
   ```

2. **Build the Application**

   ```sh
   cargo build --release
   ```

3. **Run the Kafka Services**

   Make sure you have a Kafka Services running so the application can connect and produce to it. You will only need one server that both producer and consumer will connect into.
   For testing puropse, use the provided `docker-compose.yml` file bellow to set up Kafka.
   
   ```yaml
   version: '3'
   services:
     zookeeper:
       image: zookeeper
       ports:
         - "2181:2181"
     kafka:
       image: bitnami/kafka
       ports:
         - "9092:9092"
       environment:
         KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://localhost:9092
         KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
   ```

   ```sh
   docker-compose up -d
   ```

4. **Run the Producer Application**

   ```sh
   cargo run --release
   ```

## Code Explanation

- `main.rs`

  This file sets up a Kafka producer and sends a message to a topic.

  ```rust
  use rdkafka::config::ClientConfig;
  use rdkafka::producer::{FutureProducer, FutureRecord};
  use rdkafka::util::get_rdkafka_version;
  use std::time::Duration;
  use tokio;

  #[tokio::main]
  async fn main() {
      let (version_n, version_s) = get_rdkafka_version();
      println!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

      let producer: FutureProducer = ClientConfig::new()
          .set("bootstrap.servers", "localhost:9092")
          .create()
          .expect("Producer creation error");

      let topic = "test_topic";
      let key = "key";
      let payload = "Hello, Kafka!";

      let delivery_status = producer
          .send(
              FutureRecord::to(topic)
                  .payload(payload)
                  .key(key),
              Duration::from_secs(0),
          )
          .await;

      match delivery_status {
          Ok(delivery) => println!("Delivered message to {:?}", delivery),
          Err((error, _)) => println!("Error delivering message: {:?}", error),
      }
  }
  ```

- `cargo.toml`

  ```toml
  [package]
  name = "producer_app"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  rdkafka = "0.26"
  tokio = { version = "1", features = ["full"] }
  ```
