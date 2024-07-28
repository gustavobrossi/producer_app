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
