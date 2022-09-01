use std::thread;

fn main() {
    let mut data = 100;

    thread::scope(|s|{
        s.spawn(|| {
            data = 500;
        });
        s.spawn(|| {
            data = 1000;
        });
    });
    println!("{data}");
}
