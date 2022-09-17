fn main() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    let addr: usize = unsafe { std::mem::transmute(a_ptr) };
    // let x = &a;
    let end_addr = addr + 7;
    println!("a: {a} ({a_ptr:p}...0x{end_addr:x})");
    // println!("a: {a} ({x:p})");

    println!();
    unsafe_pointer();
}

fn unsafe_pointer() {
    let ptr = 42 as *const Vec<String>;

    unsafe {
        let new_addr = ptr.offset(4);
        println!("{ptr:p} -> {new_addr:p}");
    }
}
