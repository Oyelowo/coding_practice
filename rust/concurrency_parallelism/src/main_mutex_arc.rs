use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    // Atomic Reference Counting with Arc<T>
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Mutex: {}", *counter.lock().unwrap());
}

fn main1() {
    // Using Mutexes to Allow Access to Data from One Thread at a Time
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}