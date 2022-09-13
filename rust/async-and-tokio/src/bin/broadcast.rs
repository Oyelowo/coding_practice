use std::{sync::Arc, thread::spawn};

use crossbeam::thread;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();
    let mut rx3 = tx.subscribe();

    // tx.send(20).ok();
    // tx.send(30).ok();
    // tx.send(30).ok();
    // tx.send(30).ok();
    // tx.send(30).ok();

    let p = Arc::new(tx);
    for n in 0..=50 {
        let x = p.clone();
        tokio::spawn(async move {
            x.send(n).ok();
        }).await.ok();
    }

    while let Ok(n) = rx1.recv().await {
        tokio::spawn(async move {
            dbg!(n);
        })
        .await
        .ok();
    }

    // let t1 = tokio::spawn(async move {
    //     // assert_eq!(rx1.recv().await.ok(), 10);
    //     // assert_eq!(rx1.recv().await.ok(), 20);
    //     // assert_eq!(rx1.recv().await.ok(), 30);
    //     dbg!(rx1.recv().await.ok());
    //     dbg!(rx1.recv().await.ok());
    //     dbg!(rx1.recv().await.ok());
    // })
    // .await
    // .ok();

    // let t2 = tokio::spawn(async move {
    //     dbg!(rx2.recv().await.ok());
    //     dbg!(rx2.recv().await.ok());
    //     dbg!(rx2.recv().await.ok());
    //     // assert_eq!(rx2.recv().await.ok(), 10);
    //     // assert_eq!(rx2.recv().await.ok(), 20);
    //     // assert_eq!(rx2.recv().await.ok(), 30);
    // })
    // .await
    // .ok();

    // let t3 = tokio::spawn(async move {
    //     dbg!(rx3.recv().await.ok());
    //     dbg!(rx3.recv().await.ok());
    //     dbg!(rx3.recv().await.ok());
    //     // assert_eq!(rx2.recv().await.ok(), 10);
    //     // assert_eq!(rx2.recv().await.ok(), 20);
    //     // assert_eq!(rx2.recv().await.ok(), 30);
    // })
    // .await
    // .ok();

    // tokio::spawn(async move {
    //     t1.await.ok();
    //     // t2.await.ok();
    //     // t3.await.ok();
    // })
    // .await
    // .ok();
}
