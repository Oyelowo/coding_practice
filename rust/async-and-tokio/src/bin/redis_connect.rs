use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    for i in 0..1000 {
        println!("fetching: {i}");
        let key = format!("name{i}");
        client.set(&key, format!("Oyelowo{i}").into()).await?;

        let result = client.get(&key).await?;

        println!("Get value from the server: result={result:?}");
    }

    Ok(())
}
