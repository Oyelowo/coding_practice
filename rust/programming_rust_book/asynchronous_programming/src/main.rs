use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

use crossbeam::sync::Parker;
use futures_lite::pin;
use waker_fn::waker_fn;

fn main() {
    println!("Hello, world!");
}

pub struct SpawnBlocking<T>(Arc<Mutex<Shared<T>>>);

impl<T: Send> Future for SpawnBlocking<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut guard = self.0.lock().unwrap();
        if let Some(value) = guard.value.take() {
            return Poll::Ready(value);
        }

        guard.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

/*
The Shared struct must serve as a rendezvous between the future and
the thread running the closure, so it is owned by an Arc and
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

fn block_on<F: Future>(future: F) -> F::Output {
    let parker = Parker::new();
    let unparker = parker.unparker().clone();
    let waker = waker_fn(move || unparker.unpark());
    let mut context = Context::from_waker(&waker);

    pin!(future);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => parker.park(),
        }
    }
}
