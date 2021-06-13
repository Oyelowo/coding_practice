// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use std::{
    fmt::{self, Display},
    mem,
};

fn main() {
    structures();
    enums();
    constants();
}

//////////////////////////////////////////////////////////////

// constants
// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn constants() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line
}

//////////////////////////////////////////////////////////////

// ENUMS
fn enums() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    let status = Poor;
    let status2 = Status::Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have bitcoins!"),
        Poor => println!("The poor have doge"),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Add;

    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("stringified: {}", list.stringify());
}

// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

/*
Type aliases
If you use a type alias, you can refer to each enum variant via its alias.
This might be useful if the enum's name is too long or too generic, and you want to rename it.
*/

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// use

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

/*
C-like
enum can also be used as C-like enums.
*/
// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

use crate::List::*;

#[derive(Debug)]
enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> Self {
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> Self {
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // println!("len {:?}", self);
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        // println!("self, {:?}", self);
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                // println!("head: {:?}, tail: {:?}", head, tail);
                format!("{}, {}", head, tail.stringify())
            }
            Nil => format!("Nil"),
        }
    }
}

////////////////////////////////////////////////////////////////

// STRUCTURES
fn structures() {
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

// Variable Bindings
