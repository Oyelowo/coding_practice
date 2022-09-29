use fluvio::metadata::topic::TopicSpec;
use fluvio::{self, Fluvio, FluvioConfig, FluvioError, RecordKey};

const TOPIC_NAME: &str = "hard";
const PARTITIONS: i32 = 1;
const REPLICAS: i32 = 1;

#[async_std::main]
async fn main() -> Result<(), FluvioError> {
    let fluvio_config = FluvioConfig::new("127.0.0.1:9003");
    println!("start1");
    let fluvio = Fluvio::connect_with_config(&fluvio_config).await?;
    println!("start2");
    let admin = fluvio.admin().await;
    println!("start3");
    let topic_spec = TopicSpec::new_computed(PARTITIONS, REPLICAS, None);
    println!("start4");
    let _topic_create = admin
        .create(TOPIC_NAME.to_string(), false, topic_spec)
        .await;
    println!("start5");
    let producer = fluvio.topic_producer(TOPIC_NAME).await?;
    println!("start6");
    producer.send(RecordKey::NULL, "Hello, Fluvio").await?;
    println!("start7");
    Ok(())
}
