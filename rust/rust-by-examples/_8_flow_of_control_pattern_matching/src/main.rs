fn main() {
    match_basic();
    match_tuple_destructuring();
    enums();
    pointers_ref();
    structs();
    guards();
    bindings();
}

///////////////////////////////////////////////////////////////////
// Bindings
/*
Indirectly accessing a variable makes it impossible to branch and
use that variable without re-binding. match provides the @ sigil
for binding values to names:
*/

// A function `age` which returns a `u32`.
fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn bindings() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n => println!("I'm an old person of age {:?}", n),
    }

    match some_number() {
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number.
        Some(n) => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _ => (),
    }
}

////////////////////////////////
// Guards
fn guards() {
    let pair = (2, -2);
    // TODO ^ Try different values for `pair`

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // The ^ `if condition` part is a guard
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }

    /*
    Note that the compiler does not check arbitrary expressions for whether
     all possible conditions have been checked. Therefore, you must use the _ pattern at the end.
    */

    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => println!("Fell through"), // This should not be possible to reach
    }
}

////////////////////////////////
// Structs
fn structs() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }
}

////////////////////////////////////////////////////////////////////
// pointers/ref
/*
For pointers, a distinction needs to be made between destructuring and
dereferencing as they are different concepts which are used differently from a language like C.

Dereferencing uses *
Destructuring uses &, ref, and ref mut
*/
fn pointers_ref() {
    // Assign a reference of type `i32`. The `&` signifies there
    // is a reference being assigned.
    let reference = &5;

    match reference {
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    };

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref x => {
            println!("Got a reference to a value: {:?}", x)
        }
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}

////////////////////////////////
// ENUMS
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
    HSV(u8, u8, u8),
    HSL(u8, u8, u8),
    CMY(u8, u8, u8),
    CMYK(u8, u8, u8, u8),
}
fn enums() {
    let color = Color::RGB(122, 17, 40);

    println!("What color?");

    match color {
        Color::Red => println!("This is danger"),
        Color::Green => println!("This is Green"),
        Color::Blue => println!("This is Blue"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
        // Don't need another arm because all variants have been examined
    }
}

////////////////////////////////////
// TUPLE
fn match_tuple_destructuring() {
    let triple = (0, -55, 7);
    let triple = (2, 55, 7);
    let triple = (2, 55, 7);

    println!("Tell me about {:?}", triple);

    match triple {
        (0, x, y) => println!("This is x: {:?} and y: {:?}", x, y),
        (a, b, c) if b < 0 => println!("a: {:?}, b: {:?}, c: {:?}", a, b, c),
        (1, ..) => println!("This is only x: {}", 1),
        _ => println!("Any other"),
    }
}

fn match_basic() {
    let number = 7;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);
}
