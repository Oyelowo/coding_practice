use std::{
    sync::{Arc, Mutex},
    task::Waker,
};

fn main() {
    println!("Hello, world!");
}

pub struct SpawnBlocking<T>(Arc<Mutex<Shared<T>>>);

/*
The Shared struct must serve as a rendezvous between the future and
the thread run‐ ning the closure, so it is owned by an Arc and
 protected with a Mutex. (A synchro‐ nous mutex is fine here.)
 Polling the future checks whether value is present and saves
 the waker in waker if not. The thread that runs the closure
 saves its return value in value and then invokes waker, if present.
*/
struct Shared<T> {
    value: Option<T>,
    waker: Option<Waker>,
}

pub fn spawn_blocking<T, F>(closure: F) -> SpawnBlocking<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    let inner = Arc::new(Mutex::new(Shared {
        value: None,
        waker: None,
    }));

    std::thread::spawn({
        let inner = inner.clone();
        move || {
            let value = closure();

            let maybe_waker = {
                let mut guard = inner.lock().unwrap();
                guard.value = Some(value);
                guard.waker.take()
            };
            if let Some(waker) = maybe_waker {
                waker.wake();
            }
        }
    });

    SpawnBlocking(inner)
}
