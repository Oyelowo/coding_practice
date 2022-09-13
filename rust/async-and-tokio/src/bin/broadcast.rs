use std::{fmt::format, sync::Arc, thread, time::Duration};

use tokio::sync::{
    broadcast::{channel, Sender},
    mpsc::Receiver,
};

// #[derive(Clone)]
struct Message {
    number: i32,
    responder: tokio::sync::oneshot::Sender<String>,
}

#[tokio::main]
async fn main() {
    // let kk = Arc::new(Message {number})
    let (tx, mut rx) = tokio::sync::broadcast::channel::<Arc<Message>>(3);
    for i in 0..20 {
        let tx = tx.clone();
        let (sender_oneshot, mut receiver_oneshot) = tokio::sync::oneshot::channel::<String>();
        let message = Message {
            number: i,
            responder: sender_oneshot,
        };
        let msg = Arc::new(message);
        // thread::sleep(Duration::from_secs(2));
        tokio::spawn(async move {
            // tx -> transmitter
            // rx -> receiver
            println!("pr -> {i}");

            tx.send(msg).ok();

            if let Ok(letter) = receiver_oneshot.await {
                println!("We get back, -> {letter}")
            }
            // receiver_oneshot.try_recv().ok();
            // print_and_pass(i, tx).await;
        });
    }

    // for i in 0..1000 {
    // tokio::spawn(async move {
    //     if let Ok(msg) = rx.recv().await {
    //         let mg = msg.clone();
    //         mg.responder
    //             .send(format!("Love letter from this side. {i}"));
    //         //    k.unwrap().
    //     }
    // });
    // }

    // let mut rx1 = tx.subscribe();
    // let mut rx2 = tx.subscribe();
    // let mut rx3 = tx.subscribe();
    // let mut rx1 = Arc::new(rx);
    // // let mut rx1 = Arc::new(::parking_lot::Mutex::new(rx));

    while let Ok(ref msg) = rx.recv().await {
        let n = msg.number;
        println!("I got you -----> {n}");
        let sendd = msg.clone();

        // let k = sendd.responder.clone();

        // k.send(format!("Love letter from this side. {n}")).ok();
    }

    // tokio::spawn(async move {
    // consume_and_do(rx).await;
    // })
    // .await
    // .ok();
    // println!("run1");
    // a1().await;
    // a2().await;
    // println!("run2");
    // println!("run3");
}

async fn print_and_pass(num: i32, sender: tokio::sync::mpsc::Sender<i32>) {
    // let p = (0..100_000_000).collect::<Vec<_>>();

    // for i in p {
    //     println!("pr -> {i}");
    //     sender.send(i).await.ok();
    // }

    thread::sleep(Duration::from_secs(2));
    println!("pr -> {num}");
    sender.send(num).await.ok();
}
async fn consume_and_do(mut receiver: tokio::sync::mpsc::Receiver<i32>) {
    while let Some(d) = receiver.recv().await {
        println!("I got you -----> {d}");
    }
}

async fn a1() {
    dbg!("a1");
}
async fn a2() {
    dbg!("a2");
}
