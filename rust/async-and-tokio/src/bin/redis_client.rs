use mini_redis::client;

#[tokio::main]
async fn main() {
    // Estavkush a connection to the server
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // Spawn two tasks, one gets a key , the other sets a key

    let t1 = tokio::spawn(async move {
        let res = client.get("lowo2").await;
        res
    });
    // let t2 = tokio::spawn(async move {
    //     client.set("hello", "world".into()).await;
    // });

    let d = t1.await.unwrap().unwrap();
    println!("ddd = {d:?}");
    // t2.await.unwrap();
}
