use std::thread;

const NTHREADS: u32 = 10;

// This is the `main` thread
fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];
    let v: Vec<_> = (1u8..10).map(f32::from).collect();
    let children: Vec<_> = (1u8..NTHREADS as u8).collect();

    /*     for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    } */

    children.iter().map(|i| {
        return thread::spawn(move || {
            println!("this is thread number {}", i);
        })
    });

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
