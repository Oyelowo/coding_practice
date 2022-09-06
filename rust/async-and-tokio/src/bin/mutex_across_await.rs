use std::sync::Arc;

use parking_lot::{Mutex, MutexGuard};
// use std::sync::{Arc, Mutex, MutexGuard};

use tokio::task;

// async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
// Wont work because MutexGuard is not `Send` but used across await
// which is impossible because tokio's/the executor scheduler(the state machine) will not be able to send the mutexGuard
// state to other threads during resuming of task from where it previously stopped
/* async fn increment_and_do_stuff(mutex: &::parking_lot::Mutex<i32>) {
    // let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
    let mut lock: ::parking_lot::MutexGuard<i32> = mutex.lock();
    // let mut lock = mutex.lock();
    *lock += 1;
    do_async().await;
} // Lock goes out of scope here */

// This will work becuase, we force the lock to be released after the scope `{}` before
// any .await is ever called
async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    {
        let mut lock: MutexGuard<i32> = mutex.lock();
        *lock += 1;
    }
    do_async().await;
} // Lock goes out of scope here

async fn do_async() {
    println!("Async here");
}

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(0));
    // let db = Mutex::new(0);
    for _ in 0..1000 {
        let db_spawn = db.clone();
        let o = task::spawn(async move {
            increment_and_do_stuff(&db_spawn).await;
        });
        o.await.unwrap();
    }
    let db2 = db.clone();
    println!("dddd {db2:?}");
}
