use std::{
    fmt::{self, Display},
    mem,
};

fn main() {
    // A tuple with a bunch of different types
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.7);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("long tuple second value: {}", tuple_of_tuples.1 .1);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("pair is {:?}", pair);

    let lk = reverse(pair);
    let lk = reverse((434, "gfg"));

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    //tuples can be de-structured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    /*
    Recap: Add the fmt::Display trait to the Matrix struct in the above example, so that if you switch from printing the debug format {:?} to the display format {}, you see the following output:


    ( 1.1 1.2 )
    ( 2.1 2.2 )
    */
    println!("{}", matrix);

    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));

    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array
    println!("number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // Out of bound indexing causes compile error
    // println!("{}", xs[5]);

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

// Tuples can be used as function arguments and as return values
fn reverse<T, U>(pair: (T, U)) -> (U, T) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "( {0} {1} )\n( {2} {3} )",
            self.0, self.1, self.2, self.3
        )
    }
}

fn transpose(Matrix(a, b, c, d): Matrix) -> Matrix {
    Matrix(a, c, b, d)
}

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
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
