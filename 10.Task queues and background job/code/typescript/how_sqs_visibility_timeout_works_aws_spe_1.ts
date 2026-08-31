// TypeScript, SQS producer + consumer using AWS SDK v3
import { SQSClient, SendMessageCommand, ReceiveMessageCommand, DeleteMessageCommand, Message } from "@aws-sdk/client-sqs";

interface EmailPayload {
    UserID: string;
    // other fields
}

// -- PRODUCER ------------------------------------------
export async function enqueueEmailTask(client: SQSClient, queueUrl: string, payload: EmailPayload): Promise<void> {
    const body = JSON.stringify(payload);
    await client.send(new SendMessageCommand({
        QueueUrl: queueUrl,
        MessageBody: body,
        // For FIFO queue: add MessageGroupId + MessageDeduplicationId
        // MessageGroupId: "email-group",
        // MessageDeduplicationId: payload.UserID,
    }));
}

// -- CONSUMER ------------------------------------------
export async function pollQueue(client: SQSClient, queueUrl: string): Promise<void> {
    while (true) {
        const result = await client.send(new ReceiveMessageCommand({
            QueueUrl: queueUrl,
            MaxNumberOfMessages: 10,          // batch up to 10
            WaitTimeSeconds: 20,              // long-polling reduces empty receives
            VisibilityTimeout: 60,            // 60s to process before redelivery
        }));

        const messages = result.Messages || [];
        for (const msg of messages) {
            try {
                await processMessage(msg);
                // ACK: delete from queue on success
                await client.send(new DeleteMessageCommand({
                    QueueUrl: queueUrl,
                    ReceiptHandle: msg.ReceiptHandle, // unique handle per receive
                }));
            } catch (err) {
                // On error: do nothing, visibility timeout expires, SQS redelivers
            }
        }
    }
}

async function processMessage(msg: Message): Promise<void> {
    // Process message
}
