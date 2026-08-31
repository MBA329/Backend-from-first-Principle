import { Kafka } from 'kafkajs';

const kafka = new Kafka({
  clientId: 'my-app',
  brokers: ['localhost:9092']
});

// the consumer GROUP ,  scale by adding instances
const consumer = kafka.consumer({ groupId: 'billing' });

async function run() {
  await consumer.connect();
  
  // where to start if no committed offset (sec 8)
  await consumer.subscribe({ topic: 'orders', fromBeginning: true });

  await consumer.run({
    autoCommit: false, // we'll commit manually for at-least-once (sec 9)
    eachMessage: async ({ topic, partition, message }) => {
      // do the work FIRST...
      console.log(`Processing: ${message.value?.toString()}`);
      
      // ...THEN commit the offset (at-least-once, sec 9)
      // Note: kafkajs auto-commits if you don't throw an error in eachMessage, 
      // but to do it manually:
      /*
      await consumer.commitOffsets([{
        topic,
        partition,
        offset: (BigInt(message.offset) + 1n).toString()
      }]);
      */
    },
  });
}

run().catch(console.error);
