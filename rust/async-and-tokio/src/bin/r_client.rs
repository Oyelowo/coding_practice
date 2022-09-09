use mini_redis::client::connect;

#[tokio::main]
async fn main() {
    let mut client = connect("127.0.0.1:6379").await.unwrap();
    // client.set("lowo", "dayo".into()).await.ok();

    let got = client.get("lowo").await.ok();
    println!("GOT => {got:?} ");
}
