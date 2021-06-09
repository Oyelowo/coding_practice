/*
All types which want to use std::fmt formatting traits require an
implementation to be printable. Automatic implementations are only provided
for types such as in the std library. All others must be manually implemented somehow.

The fmt::Debug trait makes this very straightforward. All types can derive
(automatically create) the fmt::Debug implementation.
This is not true for fmt::Display which must be manually implemented.
*/

// This structure cannot be printed either with `fmt::Display` or
// with `fmt::Debug`.
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);

use std::fmt;

// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.

struct Deep(Structure);

impl fmt::Debug for Deep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0 .0)
    }
}

fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`? Implement fmt::Debug explicitly rather than deriving the default behavior;
    println!("Now {:?} will print!", Deep(Structure(7)));

    // PRETTY PRINTING
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}
