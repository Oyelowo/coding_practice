use std::mem::transmute;

fn main() {
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];
    println!("{big_endian:?} vs {little_endian:?}");
    let a: i32 = unsafe { transmute(big_endian) };
    let b: i32 = unsafe { transmute(little_endian) };
    println!("{a} vs {b}")
}



