fn main() {
    if_else();
    loop_();
    nested_loop();
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
