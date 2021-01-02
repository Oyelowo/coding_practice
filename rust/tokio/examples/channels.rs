use std::io::Error;

use tokio::sync::mpsc;
 use mini_redis::{Command, client, Result};


#[tokio::main]
pub async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("sending from first handle").await;
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }

   
    // The `move` keyword is used to **move** ownership of `rx` into the task.
    
    let manager = tokio::spawn(async move  {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();


        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get {key} => {
                    client.get(&key).await;
                }
                Set {key, val}=> {
                    client.set(&key, val).await;
                }
            }
        }
    })
    Ok(())
}

