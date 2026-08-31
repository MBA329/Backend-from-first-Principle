use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer, CommitMode};
use rdkafka::Message;

#[tokio::main]
async fn main() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", "billing") // the consumer GROUP , scale by adding instances
        .set("auto.offset.reset", "earliest") // where to start if no committed offset (sec 8)
        .set("enable.auto.commit", "false") // we'll commit manually for at-least-once (sec 9)
        .create()
        .expect("Consumer creation failed");

    // Kafka assigns this instance some partitions
    consumer.subscribe(&["orders"]).expect("Can't subscribe");

    loop {
        // blocks for the next record
        match consumer.recv().await {
            Err(e) => println!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        println!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                
                // process(msg.Value) -> do the work FIRST...
                println!("Processing payload: {}", payload);

                // ...THEN commit the offset (at-least-once, sec 9)
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}
