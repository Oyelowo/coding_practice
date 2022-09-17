use std::result;

fn main() {
    let a: i32 = 40;
    let b: Box<i32> = Box::new(60);

    let result = a + *b;

    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);

    let result1 = *a + *b + *c;

    drop(a);
    let d = Box::new(1);
    let result2 = *b + *c + *d;

    println!("{result1} {result2}");
}
