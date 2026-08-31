// Rust, SQS producer + consumer using AWS SDK
use aws_sdk_sqs::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EmailPayload {
    pub user_id: String,
    // other fields
}

// -- PRODUCER ------------------------------------------
pub async fn enqueue_email_task(client: &Client, queue_url: &str, payload: &EmailPayload) -> Result<(), Error> {
    let body = serde_json::to_string(payload).unwrap();
    client.send_message()
        .queue_url(queue_url)
        .message_body(body)
        // For FIFO queue: add MessageGroupId + MessageDeduplicationId
        // .message_group_id("email-group")
        // .message_deduplication_id(&payload.user_id)
        .send()
        .await?;
    Ok(())
}

// -- CONSUMER ------------------------------------------
pub async fn poll_queue(client: &Client, queue_url: &str) -> Result<(), Error> {
    loop {
        let result = client.receive_message()
            .queue_url(queue_url)
            .max_number_of_messages(10)          // batch up to 10
            .wait_time_seconds(20)               // long-polling reduces empty receives
            .visibility_timeout(60)              // 60s to process before redelivery
            .send()
            .await?;

        if let Some(messages) = result.messages() {
            for msg in messages {
                if process_message(msg).await.is_ok() {
                    // ACK: delete from queue on success
                    if let Some(receipt_handle) = msg.receipt_handle() {
                        let _ = client.delete_message()
                            .queue_url(queue_url)
                            .receipt_handle(receipt_handle) // unique handle per receive
                            .send()
                            .await;
                    }
                }
                // On error: do nothing, visibility timeout expires, SQS redelivers
            }
        }
    }
}

async fn process_message(msg: &aws_sdk_sqs::types::Message) -> Result<(), Box<dyn std::error::Error>> {
    // Process message
    Ok(())
}
