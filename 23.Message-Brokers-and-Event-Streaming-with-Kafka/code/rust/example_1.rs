use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Using rdkafka.
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .set("acks", "all") // durability: leader + in-sync replicas (sec 5)
        .set("enable.idempotence", "true") // no duplicates on producer retry (sec 9)
        .set("linger.ms", "5") // wait up to 5ms to batch records (throughput)
        .set("compression.type", "zstd")
        .create()
        .expect("Producer creation error");

    let topic = "orders";
    
    // The KEY decides the partition -> all events for one order stay ordered (sec 4, sec 10).
    let record = FutureRecord::to(topic)
        .key("order-42")
        .payload(r#"{"event":"placed","amount":1999}"#);

    // Producing is async + batched
    match producer.send(record, Duration::from_secs(0)).await {
        Ok(delivery) => println!("Sent: {:?}", delivery),
        Err((e, _)) => println!("Error: {:?}", e),
    }

    // Flush blocks until queued messages are delivered.
    producer.flush(Duration::from_secs(5));
}
