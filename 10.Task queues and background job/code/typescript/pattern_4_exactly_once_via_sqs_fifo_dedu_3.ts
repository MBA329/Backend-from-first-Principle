// TypeScript, SQS FIFO with content-based deduplication
import { SQSClient, SendMessageCommand } from "@aws-sdk/client-sqs";

async function sendOrderMessage(client: SQSClient, payload: string, orderID: string) {
    await client.send(new SendMessageCommand({
        QueueUrl: "https://sqs.us-east-1.amazonaws.com/123/orders.fifo",
        MessageBody: payload,
        MessageGroupId: "order-processing", // ordering group
        MessageDeduplicationId: orderID,    // unique per business event
    }));
    // Sending the same orderID twice within 5 min -> second is silently dropped by SQS
}
