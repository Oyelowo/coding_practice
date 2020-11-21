use core::fmt::Debug;
use core::fmt::Formatter;
use std::{thread, time::Duration};

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector {:?}", v);
    });

    // drop(v); // oh no!

    handle.join().unwrap();
}

fn basics() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawn thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
