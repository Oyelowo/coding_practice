use std::ops::Add;

fn main() {
    let a = Point { x: 34, y: 34 };
    let b = Point { x: 34, y: 34 };
    let res = a + b;

    let a = Point { x: 34, y: 34 };
    let b = Point { x: 34, y: 34 };
    let res2 = &a + &b;

    
}

// trait PartialEq<Rhs = Self>: PartialEq<Rhs>
// where
//     Rhs: ?Sized,
// {
// }

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Add for Point {
    type Output = Line;

    fn add(self, rhs: Point) -> Self::Output {
        Line {
            start: self,
            end: rhs,
        }
    }
}

// impl Add for Point {
//     type Output = Point;

//     fn add(self, rhs: Point) -> Self::Output {
//         todo!()
//     }
// }
impl Add for &Point {
    type Output = <Point as Add>::Output;

    fn add(self, rhs: &Point) -> Self::Output {
        Point::add(*self, *rhs)
    }
}

fn sortable<T: Ord>(mut thing: Vec<T>) {
    thing.sort();
}
