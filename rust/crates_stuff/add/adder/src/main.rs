use add_one;
use doubler;
use rand;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
    println!("Hello, world! {} plus two is {}!", num, doubler::double_up(num))
}
