fn main() {
    let a: f32 = 42.42;

    let frankentype: u32 = unsafe { std::mem::transmute(a) };

    println!("{frankentype}");
    println!("{frankentype:032b}");

    let b: f32 = unsafe { std::mem::transmute(frankentype) };
    println!("{b}");
    println!("{a}, {b}");
}
