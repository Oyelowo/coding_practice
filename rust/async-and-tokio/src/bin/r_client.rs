use std::fmt::format;

use mini_redis::client::connect;
use tokio::sync::{
    mpsc,
    oneshot::{self, Sender},
};

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        response_transmitter: Sender<String>,
    },
    Set {
        key: String,
        value: String,
        response_transmitter: Sender<String>,
    },
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Command>(32);

    let tx1 = tx.clone();
    let sender = tokio::spawn(async move {
        let (txo, mut rx0) = oneshot::channel::<String>();
        let msg = Command::Set {
            key: "lowo".into(),
            value: "dayo".into(),
            response_transmitter: txo,
        };
        let (txo2, mut rx02) = oneshot::channel::<String>();
        let msg2 = Command::Set {
            key: "lowoxx".into(),
            value: "dayo".into(),
            response_transmitter: txo2,
        };
        // let t1 = tx1.send(msg.clone()).await;
        let t1 = tx1.send(msg).await;
        let t1 = tx1.send(msg2).await;
        dbg!(rx0.await.ok());
        dbg!(rx02.await.ok());
    });

    let tx2 = tx.clone();
    let getter = tokio::spawn(async move {
        let (txo, rx0) = oneshot::channel::<String>();
        let msg = Command::Get {
            key: "oye".into(),
            response_transmitter: txo,
        };
        let t1 = tx2.send(msg).await;
        dbg!(rx0.await.ok());
    });

    // let t1 = tx
    //     .send(Command::Set {
    //         key: "lowo".into(),
    //         value: "dayo".into(),
    //     })
    //     .await;
    // let t2 = tx
    //     .send(Command::Set {
    //         key: "lowo".into(),
    //         value: "dayo".into(),
    //     })
    //     .await;
    // let t3 = tx.send(Command::Get { key: "lowo".into() }).await;
    // let mut client = connect("127.0.0.1:6379").await.unwrap();
    // // client.set("lowo", "dayo".into()).await.ok();

    // let got = client.get("lowo").await.ok();

    while let Some(message) = rx.recv().await {
        // dbg!(message);
        match message {
            Command::Get {
                key,
                response_transmitter,
            } => {
                // dbg!(key);
                response_transmitter.send("From a getter Responder".to_string()).ok();
            }
            Command::Set {
                key,
                value,
                response_transmitter,
            } => {
                let msg = format!("I got this buddy setter Responder: {key} => {value}");
                response_transmitter.send(msg).ok();
            }
        }
    }
    // println!("GOT => {got:?} ");
    // sender.await;
    // getter.await;
}
