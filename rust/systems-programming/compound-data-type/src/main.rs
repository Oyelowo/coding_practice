// use rand::random;

static mut ERROR: i32 = 0;

fn read() {
    if random() {
        unsafe {
            ERROR = 1;
        }
    }
}

fn random() -> bool {
    todo!()
}

fn main() {
    let p = vec![12];
    // p.app
    let uz = usize::MAX;
    let u6 = u64::MAX;

    println!("Hello, world!:uz -> {uz}, u6 -> {u6}");

    unsafe {
        if ERROR != 0 {
            panic!("An error has occured while reading the file");
        }
    }
}
