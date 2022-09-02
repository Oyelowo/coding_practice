use std::{thread, num};

fn main() {
    let p = 34.4453_f32.round();
    println!("Roundded: {p}");

    let three = 0b11; // Binary
    let thirty = 0o36;  // Octal
    let three_hundred = 0x12C; // Hexadecimal

    println!("base 10: {three} {thirty} {three_hundred}");
    println!("base 2: {three:b} {thirty:b} {three_hundred:b}");
    println!("base 8: {three:o} {thirty:o} {three_hundred:o}");
    println!("base 16: {three:x} {thirty:x} {three_hundred:x}");
    
    // let mut data = 100;

    // thread::scope(|s|{
    //     s.spawn(|| {
    //         data = 500;
    //     });
    //     s.spawn(|| {
    //         data = 1000;
    //     });
    // });
    // println!("{data}");
}
