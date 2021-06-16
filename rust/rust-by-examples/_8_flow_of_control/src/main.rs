use std::fmt::format;

fn main() {
    if_else();
    loop_();
    nested_loop();
    returning_from_loops();
    while_loop();
    for_loop();
    iterators();
}

////////////////////////////////////////////////////////////////////////////////
// for and iterators
/*
The for in construct is able to interact with an Iterator in several ways.
As discussed in the section on the Iterator trait, by default the for loop
will apply the into_iter function to the collection. However,
this is not the only means of converting collections into iterators.

into_iter, iter and iter_mut all handle the conversion of a collection
into an iterator in different ways, by providing different views on the data within.

iter - This borrows each element of the collection through each iteration.
Thus leaving the collection untouched and available for reuse after the loop.
*/
fn iterators() {
    let names = vec!["Oyelowo", "Oyedayo", "Maria"];
    for name in names.iter() {
        match name {
            &"Oyelowo" => println!("This is the King"),
            _ => println!("Others"),
        }
    }
    println!("names: {:?}", names);

    /*
    into_iter - This consumes the collection so that on each iteration
    the exact data is provided. Once the collection has been consumed
    it is no longer available for reuse as it has been 'moved' within the loop.
    */

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // println!("names: {:?}", names);
    // FIXME ^ Comment out this line

    /*
    iter_mut - This mutably borrows each element of the collection,
    allowing for the collection to be modified in place.
    */
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);

    /*
    In the above snippets note the type of match branch, that is the key difference in the types of iteration. The difference in type then of course implies differing actions that are able to be performed.
    */
}

////////////////////////////////
// For loops
/*
for and range
The for in construct can be used to iterate through an Iterator.
One of the easiest ways to create an iterator is to use the range
notation a..b. This yields values from a (inclusive) to b (exclusive) in steps of one
*/
fn for_loop() {
    for n in -30..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    /*
    Alternatively, a..=b can be used for a range that
    is inclusive on both ends. The above can be written as:
    */
    for n in -30..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

////////////////////////////////////
// While
fn while_loop() {
    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }
}

/////////////////////////////////////
// Returning from loops
/*
One of the uses of a loop is to retry an operation until it succeeds.
If the operation returns a value though, you might need to pass
it to the rest of the code: put it after the break, and it will be
returned by the loop expression.
*/

fn returning_from_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("I am running the counter: {}", counter);

        if counter == 50 {
            break format!("This is returned {}", counter * 4);
        }
    };
    println!("The result returning from the loop, {}", result);
}

////////////////////////////////
// Nesting and labels
/*
It's possible to break or continue outer loops when dealing
with nested loops. In these cases, the loops must be annotated
with some 'label, and the label must be passed to the break/continue statement.
*/
fn nested_loop() {
    let mut counter = 0;
    'outer: loop {
        counter += 1;
        println!("This is the outer, counter {}", counter);

        /*      if counter > 50 {
            break 'outer;
        } */
        let mut counter2 = 0;

        'inner: loop {
            counter2 += 1;
            println!("This is the inner, counter {}", counter2);

            if counter2 > 50 {
                println!("This breaks inner");
                break;
            }

            if counter2 + counter > 100 {
                println!("This breaks outer");
                break 'outer;
            }
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}

//////////////////////////////////////////////////////////////
// LOOP
/*
Rust provides a loop keyword to indicate an infinite loop.

The break statement can be used to exit a loop at anytime,
whereas the continue statement can be used to skip the rest
of the iteration and start a new one.
*/
fn loop_() {
    let mut counter = 0u32;

    loop {
        counter += 1;

        println!("Counter is {}", counter);

        if counter % 2 == 0 {
            continue;
        }
        println!("Below Counter: {}", counter);

        if counter > 10 {
            break;
        }
    }
}

//////////////////////////////////////////////////////////////
/// IF ELSE
fn if_else() {
    let n = -95;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // This expression returns an `i32`.
        (10 * n) as f32
    } else {
        println!(", and is a big number, halve the number");

        // This expression must return an `i32` as well.
        (n as f32 / 2f32) as f32

        // TODO ^ Try suppressing this expression with a semicolon.
    };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);

    let big_n = match n {
        x if x < 10 && x > -10 => {
            println!(", and is a thing");
            x as f32 * 10f32
        }
        n => {
            println!("Others");
            n as f32 / 2f32
        }
    };

    println!("second: {} -> {}", n, big_n);
    let big_n = match n {
        x @ -10..=10 => {
            println!(", and is a thing");
            x as f32 * 10f32
        }
        d @ _ => {
            println!("Others");
            d as f32 / 2f32
        }
    };

    println!("third: {} -> {}", n, big_n);
}
