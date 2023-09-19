import Fluvio, { RecordSet } from '@fluvio/client';

// define the name of the topic to use
const TOPIC_NAME = "my-topic"


// define the partition where the topic
// records will be stored;
const PARTITION = 0

// Instantiate a new fluvio client
const fluvio = await Fluvio.connect();

//// Fluvio Admin Client

// Create a new Fluvio Admin Client to manage
// topics and partitions
const admin = await fluvio.admin();

// Create a new topic
await admin.createTopic(TOPIC_NAME)

//// Topic Producer

// Create a topic producer for the topic;
const producer = await fluvio.topicProducer(TOPIC_NAME);

// Send a new topic record
producer.sendRecord("stringified data", PARTITION)

//// Partition Consumer

// Instantiate a new topic listener;
const consumer = await fluvio.partitionConsumer(TOPIC_NAME, PARTITION)

// Listen for new topics sent by a topic producer;
await consumer.stream(async (data: string) => {
    // handle data record
})