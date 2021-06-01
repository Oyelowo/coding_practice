//!

use std::ops::{Add, Deref, Sub};
// #![warn(missing_copy_implementations, rust_2018_idioms, missing_docs)]

pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            // TODO: bug
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            // &'a' str =&'static str
            Some(rest)
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    // let letters: Vec<_> = StrSplit::new(haystack, " ").collect::<Vec<_>>();
    let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();

    /*    for letter in StrSplit::new(haystack, " ") {

    } */
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    // Alternative way
    /*  let letters = StrSplit::new(haystack, " ");

    /*    for letter in StrSplit::new(haystack, " ") {

    } */
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter())); */
}

struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // Static method signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Instance method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Sheep {
        Sheep {
            name: name,
            naked: false,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }
    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main2() {
    // Type annotation is necessary in this case.
    let mut dolly: Sheep = Animal::new("Dolly");

    // TODO ^ Try removing the type annotations.

    dolly.talk();
    dolly.shear();
    dolly.talk();
}

// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd, Debug)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

impl Add<Centimeters> for Inches {
    type Output = Centimeters;
    fn add(self, other: Centimeters) -> Centimeters {
        let k = self.to_centimeters().0 + other.0;
        Centimeters(k)
    }
}

impl Sub<Centimeters> for Inches {
    type Output = Centimeters;
    fn sub(self, other: Centimeters) -> Centimeters {
        let k = self.to_centimeters().0 + other.0;
        Centimeters(k)
    }
}

fn jtest(other: impl Sub) -> Centimeters {
    Centimeters(6.9)
}
// `Seconds`, a tuple struct with no additional attributes
struct Seconds(i32);

fn getIt() -> Centimeters {
    let height = Centimeters(10.0);
    let hh_in_cm = 10;

    let width = Inches(500);

    let total = width + height;
    println!("hgfghi{}", total.0);
    total
}

#[test]
fn test_2() {
    assert_eq!(getIt(), Centimeters(1280.0));
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
struct Ref<'a, T>(&'a T);

#[test]
fn test_geom() {
    let point1 = Point::new(2.34, 5.7);
    let point2 = Point::new(42.64, 75.3);

    let pointDiff = point2 - point1;

    println!("{:?}", pointDiff);

    assert_eq!(pointDiff.x, 40.3);
}

trait Red {}

struct Ball<'a> {
    diameter: &'a i32,
}

fn main() {
    let num = 5;
}

// blog: create post,
// add more text,
// request review,
// approve,
// published

struct Post {
    content: String,
}

struct DraftPost {
    content: String,
}

struct PendingReviewPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

#[test]
fn test_state() {
    let mut post = Post::new();
    post.add_text("New text 1");
    post.add_text("New text 2");
    post.add_text("New text 3");


    let post = post.request_review();
    let post = post.approve();
    println!("{}", post.content());

    assert_eq!(post.content(), "New text 1New text 2New text 3");
}
