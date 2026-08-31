// Rust, SQS FIFO with content-based deduplication
use aws_sdk_sqs::Client;

async fn send_order_message(client: &Client, payload: &str, order_id: &str) -> Result<(), aws_sdk_sqs::Error> {
    client.send_message()
        .queue_url("https://sqs.us-east-1.amazonaws.com/123/orders.fifo")
        .message_body(payload)
        .message_group_id("order-processing") // ordering group
        .message_deduplication_id(order_id)    // unique per business event
        .send()
        .await?;
    // Sending the same orderID twice within 5 min -> second is silently dropped by SQS
    Ok(())
}
