use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;
use std::time::Duration;
use tokio;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let kafka_server_url = env::var("KAFKA_SERVER_URL").unwrap_or_else(|_| {
        println!("Warning: KAFKA_SERVER_URL is not set. Using localhost as default.");
        "localhost:9092".to_string()
    });

    let (version_n, version_s) = get_rdkafka_version();
    println!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &kafka_server_url)
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