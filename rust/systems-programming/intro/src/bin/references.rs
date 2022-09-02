fn main() {
    let mut a = 42;
    let r = &mut a;
    // let b = a + *r;

    *r = 32;
    println!("xxx{a}");
    // println!("a + a = {b}");
}
