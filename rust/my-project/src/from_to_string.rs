use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;

#[derive(Debug)]
struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}


impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',')
                                 .collect();

        Ok(Circle { radius: coords.last().unwrap()})
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
    
    let kk = circle.to_string();
    
    println!("My set circle {:?}", Circle::from_str(&kk).unwrap());


    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
