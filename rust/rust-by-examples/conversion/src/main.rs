use std::convert::{From, TryFrom, TryInto};

fn main() {
    from();
    even_number();
    convert_to_string();
    parsing_a_string();
}

/// TryFrom and TryInto

/*
Similar to From and Into, TryFrom and TryInto are
generic traits for converting between types. Unlike From/Into,
the TryFrom/TryInto traits are used for fallible conversions, and as such, return Results.
*/

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn even_number() {
    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(9), Err(()));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));

    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

////////////////////////////////

// FROM and Into
/*

The From and Into traits are inherently linked, and
this is actually part of its implementation. If you are
able to convert type A from type B, then it should be easy
to believe that we should be able to convert type B to type A.

From
The From trait allows for a type to define how to create
itself from another type, hence providing a very simple mechanism
for converting between several types. There are numerous implementations
of this trait within the standard library for conversion of primitive and common types.

For example we can easily convert a str into a String
*/
fn from() {
    let my_str = "Hello, world";
    let my_string = String::from(my_str);

    let number = Number::from(50);
    println!("My number is {:?}", number);

    // Into
    /*
        The Into trait is simply the reciprocal of the From trait. That is,
        if you have implemented the From trait for your type, Into will call it when necessary.

    Using the Into trait will typically require specification of the
    type to convert into as the compiler is unable to determine this most of the time. However this is a small trade-off considering we get the functionality for free.
        */
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);

    let num: Number = 5.5.into();
    println!("My number is fraction {:?}", num);
}

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl From<f32> for Number {
    fn from(item: f32) -> Self {
        Number {
            value: (item * 2.0) as i32,
        }
    }
}

// To and from Strings

/*
Converting to String
To convert any type to a String is as simple as implementing the
ToString trait for the type. Rather than doing so directly,
you should implement the fmt::Display trait which automagically
provides ToString and also allows printing the type as discussed in the section on print!.
*/

use std::fmt;
use std::str::FromStr;

struct Circle {
    radius: i32,
}

/*
No need for this if you just implement Display as below
impl ToString for Circle {
    fn to_string(&self) -> String {
        todo!()
    }
}
 */
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn convert_to_string() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
    println!("Also works for normal display::: {}", circle);
}

// Parsing a String
/*
One of the more common types to convert a string into is a number.
he idiomatic approach to this is to use the parse function and either
to arrange for type inference or to specify the type to parse
using the 'turbofish' syntax. Both alternatives are shown in the following example.

This will convert the string into the type specified so long
as the FromStr trait is implemented for that type. This is
implemented for numerous types within the standard library.
To obtain this functionality on a user defined type simply
implement the FromStr trait for that type.
*/

struct Student {
    name: String,
    age: u8,
}

fn parsing_a_string() {
    let parsed: i32 = "5".parse().expect("Couldn't parse the striing");
    let parsed_with_turbo_fish = "50".parse::<i32>().expect("Couldn't parse the striing");

    let sum = parsed + parsed_with_turbo_fish;
    println!("Sum: {:?}", sum);
}

// Expressions

/*
Blocks are expressions too, so they can be used as values in assignments. The last expression in the block will be assigned to the place expression such as a local variable. However, if the last expression of the block ends with a semicolon, the return value will be ().
*/
fn expression() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
