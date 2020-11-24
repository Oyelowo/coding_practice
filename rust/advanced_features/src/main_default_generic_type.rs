use std::ops::{Add, Div, Mul, Sub};

// Default Generic Type Parameters and Operator Overloading
#[derive(PartialEq, Debug)]
struct Millimeters(u32);

#[derive(PartialEq, Debug)]
struct Meters(u32);

// default type parameters. 
/*
Rhs : Right hand side
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
} */

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

impl Add<Millimeters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Self::Output {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl Div for Point {
    type Output = Point;

    fn div(self, other: Point) -> Self::Output {
        Point::new(self.x / other.x, self.y / other.y)
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Self::Output {
        Point::new(self.x * other.x, self.y * other.y)
    }
}

fn main() {
    assert_eq!(Point::new(1, 0) + Point::new(2, 3), Point::new(3, 3));
    assert_eq!(Point::new(1, 0) - Point::new(2, 3), Point::new(-1, -3));
    assert_eq!(
        Point::new(1, 0) / Point::new(2, 3),
        Point::new((0.5 as u8).into(), 0)
    );
    assert_eq!(Point::new(1, 0) * Point::new(2, 3), Point::new(2, 0));

    assert_eq!(Millimeters(10) + Meters(4), Millimeters(4010));
    assert_eq!(Millimeters(10) + Millimeters(42), Millimeters(52));
}
