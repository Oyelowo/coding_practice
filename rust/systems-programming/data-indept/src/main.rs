fn main() {
    // past_a_range();
    // float_bit_str_as_int();
    isolate_signed_bits_from_f32();
    isolate_exponent_from_f32();
    isolate_and_decode_mantissa();
    // let p = 0b1000 >> 2;
    // dbg!ppp; {p);
    // let p = 2u32.pow(-0);
}

fn isolate_and_decode_mantissa() {
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    let n_bits = 10027796;
println!("{n_bits:0b}");
    let mut mantissa: f32 = 1.0;

    // for i in 0..23 {
    for i in 0..=22 {
        // dbg!(i);

        let mask = 1 << i;
        // dbg!(mask);

        let one_at_bit_i = n_bits & mask;
        dbg!(one_at_bit_i);

        if one_at_bit_i != 0 {
            let i_ = i as f32;
            // dbg!(i_);

            let weight = 2_f32.powf(i_ - 23.0);
            // dbg!(weight);

            mantissa += weight;
            // dbg!(mantissa);
        }
    }
    dbg!(mantissa);
}

fn isolate_exponent_from_f32() {
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits(); // Interpret f32 as u32 for bit manipulation
    let exponent_ = n_bits >> 23; // Overwrite the mantissa
    let exponent_ = exponent_ & 0xff; // Filter the sign
    let exponent_ = (exponent_ as i32) - 127; //  Interpret the bits as signed integer and sub the bias

    dbg!(exponent_);
}

fn isolate_signed_bits_from_f32() {
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    let sign_bit = n_bits >> 31;
    dbg!(sign_bit);
}

fn past_a_range() {
    let mut i: u16 = 0;
    dbg!(i);

    loop {
        i += 1000;
        dbg!(i);
        if i % 1000 == 0 {
            dbg!();
        }
    }
}

// #[allow(arithmetic_overflow)]
fn float_bit_str_as_int() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    // These two values have the same bit pattern but different types.
    println!("{a}: {a:016b} {a}");
    println!("{b}: {a:016b} {b}");

    let a = 42.42f32;

    let frankentype: u32 = unsafe { std::mem::transmute(a) };

    dbg!(frankentype);
    println!("{frankentype:032b}");

    let b: f32 = unsafe { std::mem::transmute(frankentype) };

    dbg!(b);
    assert_eq!(a, b);

    let max_u16 = 0b1111_1111_1111_1111u16;
    dbg!(max_u16);

    // Overflows
    // let (a, b) = (200, 200);
    // let c: u8  = a + b;
    // dbg!200 + 200 = {c);
}
