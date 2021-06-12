use std::{
    fmt::{self, Display},
    mem,
};

fn main() {
    let name = String::from("Oyelowo");
    let age = 99;
    let password = String::from("1234567890");
    // shorthand is used here to initialize the properties
    let oyelowo = Person {
        name,
        age,
        password,
    };

    println!("{:?}", oyelowo);

    let point: Point = Point { x: 20.5, y: 0.5 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {
        x: top_edge,
        y: left_edge,
    } = point;

    let rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // Activity
    /*
        Add a function rect_area which calculates the area of a rectangle
        (try using nested destructuring).

    */
    println!("Area of the rectangle: {} sqm", rectangle.rect_area());

    /*
    Add a function square which takes a Point and a f32 as arguments,
    and returns a Rectangle with its lower left corner on the point,
    and a width and height corresponding to the f32.
    */
    println!(
        "Area of the rectangle: {:?} sqm",
        square(point, 10.5).rect_area()
    );
}

struct Person {
    name: String,
    age: u8,
    password: String,
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let password_silenced = "*".repeat(self.password.len());
        write!(
            f,
            "{{\n name: {},\n age: {},\n password: {},\n}}",
            self.name, self.age, password_silenced
        )
    }
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// structs can also be used in another
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        // L * W
        let Rectangle {
            bottom_right: Point { x: x2, y: y1 },
            top_left: Point { x: x1, y: y2 },
        } = self;
        // let length = self.bottom_right.x - self.top_left.x;
        //  let width = self.top_left.y - self.bottom_right.y;
        let length = x2 - x1;
        let width = y2 - y1;
        length * width
    }

    fn square(Point { x, y }: Point, size: f32) -> Self {
        Self {
            top_left: Point { x, y: y + size },
            bottom_right: Point { x: x + size, y },
        }
    }
}

fn square(Point { x, y }: Point, size: f32) -> Rectangle {
    Rectangle {
        top_left: Point { x, y: y + size },
        bottom_right: Point { x: x + size, y },
    }
}

/*
top_left= (x, lower_left.y + size)


lower_left (x,y)                                bottom_right: (x + size, y)
*/
