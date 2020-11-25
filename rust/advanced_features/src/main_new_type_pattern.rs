use std::fmt;

// Using the Newtype Pattern to Implement External Traits on External Types

fn main() {
    let w = Wrapper(vec![String::from("hello"), "there".to_string()]);
    println!("w = {}", w);
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
