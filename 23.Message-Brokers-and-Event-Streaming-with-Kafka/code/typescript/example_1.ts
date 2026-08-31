import { Kafka, CompressionTypes } from 'kafkajs';

const kafka = new Kafka({
  clientId: 'my-app',
  brokers: ['localhost:9092']
});

const producer = kafka.producer({
  maxInFlightRequests: 1,
  idempotent: true // no duplicates on producer retry (sec 9)
});

async function run() {
  await producer.connect();

  const topic = "orders";
  // The KEY decides the partition -> all events for one order stay ordered (sec 4, sec 10).
  await producer.send({
    topic: topic,
    compression: CompressionTypes.ZSTD,
    messages: [
      {
        key: 'order-42',
        value: JSON.stringify({event: 'placed', amount: 1999})
      }
    ],
    acks: -1, // durability: leader + in-sync replicas (sec 5)
  });

  await producer.disconnect();
}

run().catch(console.error);
