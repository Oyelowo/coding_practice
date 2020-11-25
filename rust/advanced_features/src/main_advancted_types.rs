use std::{fmt, io::Error};

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));

    let name = "Oye";
    let name2 = "Oye".to_string();
    let k = generic(name);

}

fn takes_long_type(f: Thunk) {}
// fn returns_long_type() -> Thunk {}

// Creating Type Synonyms with Type Aliases
type Thunk = Box<dyn Fn() + Send + 'static>;

type Kilometers = i32;

type Result2<T> = Result<T, Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
pub trait Write2 {
    fn write(&mut self, buf: &[u8]) -> Result2<usize>;
    fn flush(&mut self) -> Result2<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result2<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result2<()>;
}

//  The Never Type that Never Returns

fn bar() -> ! {
    print!("forever ");

    loop {
        println!("55");
    }
}

/* impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
 */

// To work with DSTs, Rust has a particular trait called the Sized trait to determine whether or not a type’s size is known at compile time. This trait is automatically implemented for everything whose size is known at compile time. In addition, Rust implicitly adds a bound on Sized to every generic function. That is, a generic function definition like this:

fn generic<T>(t: T) -> T {
    // --snip--
    t
}
// is actually treated as though we had written this:

/* fn generic<T: Sized>(t: T) {
    // --snip--
} */

// By default, generic functions will work only on types that have a known size at compile time. However, you can use the following special syntax to relax this restriction:

fn generic2<T: ?Sized>(t: &T) -> &T {
    t
}
