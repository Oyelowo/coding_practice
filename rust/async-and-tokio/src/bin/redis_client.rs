use mini_redis::client;

#[tokio::main]
async fn main() {
    // Estavkush a connection to the server
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // Spawn two tasks, one gets a key , the other sets a key

    let t1 = tokio::spawn(async move {
        let res = client.get("hello").await;
    });
    let t2 = tokio::spawn(async move {
        client.set("hello", "world".into()).await;
    });

    t1.await.unwrap();
    t2.await.unwrap();
}
