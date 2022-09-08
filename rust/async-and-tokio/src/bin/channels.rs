use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

/*
The mpsc channel is used to send commands to the task managing the redis connection.
The multi-producer capability allows messages to be sent from many tasks.

The channel is created with a capacity of 32. If messages are sent faster than they are received,
 the channel will store them. Once the 32 messages are stored in the channel,
 calling send(...).await will go to sleep until a message has been removed by the receiver.
 */
#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Command>(32);

    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // Starts receiving messages
        while let Some(m) = rx.recv().await {
            match m {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    resp.send(res).ok();
                }
                Command::Set { key, value, resp } => {
                    let res = client.set(&key, value).await;
                    resp.send(res).ok();
                }
            }
        }
    });

    let t1 = tokio::spawn(async move {
        let (txo, rxo) = oneshot::channel();
        let cmd = Command::Get {
            key: "lowo".to_string(),
            resp: txo, // value: "dayo".into(),
        };

        tx.send(cmd).await.unwrap();
        let rcv = rxo.await;
        println!("GOT = {rcv:?}");
    });

    let t2 = tokio::spawn(async move {
        let (txo, rxo) = oneshot::channel();
        // let tx2 = tx2.clone();
        let cmd = Command::Set {
            key: "lowo".to_string(),
            value: "dayo".into(),
            resp: txo,
        };

        tx2.send(cmd).await.unwrap();
        let res = rxo.await;
        println!("GOT = {res:?}");
    });

    // tx2.send(Command::Get { key: "lowo".into() }).await.unwrap();

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}

async fn basics() {
    // Create a new channel with a capacity of at most 32.
    let (tx, mut rx) = mpsc::channel(32);
    // Both messages are sent to the single Receiver handle. It is not possible to clone the receiver of an mpsc channel.

    let tx2 = tx.clone();
    tokio::spawn(async move {
        tx.send("sending from first handle").await;
    });

    tokio::spawn(async move {
        tx2.send("Sending from second handle").await;
    });

    while let Some(v) = rx.recv().await {
        println!("GOT = {v}")
    }
}

type Key = String;
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;
#[derive(Debug)]
enum Command {b
    Get {
        key: Key,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: Key,
        value: Bytes,
        resp: Responder<()>,
    },
}
