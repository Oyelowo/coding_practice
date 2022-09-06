
// #[allow(arithmetic_overflow)]
fn main() {

    let a: u16 = 50115;
    let b: i16 = -15421;

    // These two values have the same bit pattern but different types.
    println!("a: {a:016b} {a}");
    println!("b: {b:016b} {b}");

    let a = 42.42f32;

    let frankentype: u32 = unsafe { std::mem::transmute(a) };

    println!("{frankentype}");
    println!("{frankentype:032b}");

    let b: f32 = unsafe { std::mem::transmute(frankentype) };

    println!("{b}");
    assert_eq!(a, b);


    let max_u16 = 0b1111_1111_1111_1111u16;
    println!("max_{max_u16}");

    // Overflows
    // let (a, b) = (200, 200);
    // let c: u8  = a + b;
    // println!("200 + 200 = {c}");
}
