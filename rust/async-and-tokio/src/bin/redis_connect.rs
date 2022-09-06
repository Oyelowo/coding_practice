use std::sync::Arc;

use mini_redis::{client, Result};
use tokio::task;

#[tokio::main]
async fn main() -> Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    let client1 = Arc::new(::tokio::sync::Mutex::new(client));

    for i in 0..1000000 {
        let client_copy = client1.clone();
        task::spawn(async move {
            let mut client_lock = client_copy.lock().await;
            println!("fetching: {i}");
            let key = format!("name{i}");
            client_lock
                .set(&key, format!("Oyelowo{i}").into())
                .await
                .unwrap();

            let result = client_lock.get(&key).await.unwrap();

            println!("Get value from the server: result={result:?}");
        });
    }

    Ok(())
}
