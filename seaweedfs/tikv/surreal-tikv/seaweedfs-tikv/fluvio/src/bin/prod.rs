use async_std::stream::StreamExt;
use chrono::Local;
use fluvio::metadata::topic::TopicSpec;
use fluvio::{
    self, Compression, Fluvio, FluvioAdmin, FluvioConfig, RecordKey, TopicProducerConfigBuilder,
};

  #[derive(Clone, Default)]
    pub struct FluvioWebsocketConnector {}
    impl FluvioWebsocketConnector {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[async_trait(?Send)]
    impl TcpDomainConnector for FluvioWebsocketConnector {
        async fn connect(
            &self,
            addr: &str,
        ) -> Result<(BoxWriteConnection, BoxReadConnection, ConnectionFd), IoError> {
            let addr = if addr == "localhost:9010" {
                "ws://localhost:3001"
            } else {
                addr
            };

            let (mut _ws, wsstream) = WsMeta::connect(addr, None)
                .await
                .map_err(|e| IoError::new(std::io::ErrorKind::Other, e))?;
            let wsstream_clone = wsstream.clone();
            Ok((
                Box::new(wsstream.into_io()),
                Box::new(wsstream_clone.into_io()),
                String::from(addr),
            ))
        }

        fn new_domain(&self, _domain: String) -> DomainConnector {
            Box::new(self.clone())
        }

        fn domain(&self) -> &str {
            "localhost"
        }


#[async_std::main]
async fn main(){
     let config = FluvioConfig::new("ws://localhost:3000");
        let client =
            Fluvio::connect_with_connector(Box::new(FluvioWebsocketConnector::new()), &config)
                .await;
        assert!(client.is_ok());
        let client = client.unwrap();
        let mut admin = client.admin().await;
        let topic = "wasm-test-produce-consume".to_string();
        /*
        let err = admin.create(topic, false, TopicSpec::default()).await;
        tracing::error!("ERROR: {:?}", err);
        assert!(err.is_ok())
        */
        let producer = client.topic_producer(topic.clone()).await;
        assert!(producer.is_ok());
        let producer = producer.unwrap();
        let send = producer.send("foo", "bar").await;

        let consumer = client.partition_consumer(topic, 0).await;
        assert!(consumer.is_ok());
        let consumer = consumer.unwrap();

        let stream = consumer.stream(crate::Offset::beginning()).await;
        assert!(stream.is_ok());
        let mut stream = stream.unwrap();

        stream.next().await;

        for i in 1..10 {
            let key = format!("key-{}", i);
            let value = format!("value-{}", i);
            let send = producer.send(key.clone(), value.clone()).await;
            assert!(send.is_ok());
            let next = stream.next().await;
            assert!(next.is_some());
            let next = next.unwrap();
            assert!(next.is_ok());
            let next = next.unwrap();
            assert_eq!(
                String::from_utf8_lossy(next.key().unwrap()).to_string(),
                key
            );
}