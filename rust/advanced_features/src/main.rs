use std::fmt;

// Using Supertraits to Require One Trait’s Functionality Within Another Trait

fn main() {
    let p = Point::new(64, 342444);
    p.outline_print();
    println!("{}", p.to_string().len());
    println!("{}", p.to_string());
}

// fmt::Display has to be implemented for every type that OutlinePrint is implemented for
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// This won't work without the below
impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
