fn main() {
    macro_rules::main();
    designators::main();
    overload::main();
    repeat::main();
}

mod macro_rules {
    /*
        Rust provides a powerful macro system that allows metaprogramming.
        As you've seen in previous chapters, macros look like functions,
         except that their name ends with a bang !, but instead of generating
         a function call, macros are expanded into source code that gets compiled
          with the rest of the program. However, unlike macros in C and other
          languages, Rust macros are expanded into abstract syntax trees,
          rather than string preprocessing, so you don't get unexpected precedence bugs.

    Macros are created using the macro_rules! macro.
        */

    // This is a simple macro named `say_hello`.

    macro_rules! say_something {
        () => {
            // The macro will expand into the contents of this block.
            println!("Oyelowo says something");
            (0..=2).for_each(|i| println!("Oyelowo says, {}", i));
        };
    }

    pub fn main() {
        say_something!();
    }

    /*
        So why are macros useful?

    Don't repeat yourself. There are many cases where you may need similar
    functionality in multiple places but with different types. Often,
    writing a macro is a useful way to avoid repeating code. (More on this later)

    Domain-specific languages. Macros allow you to define special syntax
    for a specific purpose. (More on this later)

    Variadic interfaces. Sometimes you want to define an interface that takes a variable number of arguments. An example is println! which could take any number of arguments, depending on the format string!. (More on this later)
        */
}

// Syntax
/*
In following subsections, we will show how to define macros in Rust. There are three basic ideas:

Patterns and Designators
Overloading
Repetition
*/

mod designators {
    // The arguments of a macro are prefixed by a dollar sign $ and type
    // annotated with a designator:

    macro_rules! create_function {
        // This macro takes an argument of designator `ident` and
        // creates a function named `$func_name`.
        // The `ident` designator is used for variable/function names.
        ($funct_name:ident) => {
            fn $funct_name() {
                // The `stringify!` macro converts an `ident` into a string.

                println!("You called {:?}()", stringify!($funct_name));
            }
        };
    }

    // Create functions named `foo` and `bar` with the above macro.
    create_function!(foo);
    create_function!(bar);

    macro_rules! print_result {
        // This macro takes an expression of type `expr` and prints
        // it as a string along with its result.
        // The `expr` designator is used for expressions.
        ($expression:expr) => {
            // `stringify!` will convert the expression *as it is* into a string.
            println!("{:?} = {:?}", stringify!($expression), $expression);
        };
    }

    pub fn main() {
        foo();
        bar();
        print_result!(1u32 + 1);

        // Recall that blocks are expressions too!

        print_result!({
            let a = 54;

            let c: i32 = {
                let b = 56;
                b * 5
            };

            a * c
        });

        print_result!({
            let x = 1u32;

            x * x + 2 * x - 1
        });
    }

    /*
        These are some of the available designators:

    block
    expr is used for expressions
    ident is used for variable/function names
    item
    literal is used for literal constants
    pat (pattern)
    path
    stmt (statement)
    tt (token tree)
    ty (type)
    vis (visibility qualifier)
        */
}

/////////////////////////////////////
// Overload
/*
Overload
Macros can be overloaded to accept different combinations of arguments.
In that regard, macro_rules! can work similarly to a match block:
*/
mod overload {
    // `test!` will compare `$left` and `$right`
    // in different ways depending on how you invoke it:

    macro_rules! test {
        // Arguments don't need to be separated by a comma.
        // Any template can be used!
        ($left: expr; and $right: expr) => {
            println!(
                "{:?} and {:?} is  {:?}",
                stringify!($left),
                stringify!($right),
                $left && $right
            )
        }; // ^ each arm must end with a semicolon.
        ($left: expr; or $right: expr) => {
            println!(
                "{:?} or {:?} is {:?}",
                stringify!($left),
                stringify!($right),
                $left || $right
            )
        };
        ($left: expr; hihi $right: expr) => {
            println!("Pump it")
        };
    }

    macro_rules! assert_equal_custom {
        ($left: expr, $right: expr) => {
            if $left == $right {
                println!("Good job! {:?} is truly equal to {:?}", $left, $right);
            } else {
                panic!(
                    "Bad job!, {:?} and {:?} is {:?}",
                    stringify!($left),
                    stringify!($right),
                    $left == $right
                );
            }
        };
    }

    pub fn main() {
        test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
        test!(true; or false);
        test!(4; hihi 4);
        assert_equal_custom!(5, 5);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Repeat
/*
Macros can use + in the argument list to indicate that an argument
may repeat at least once, or *, to indicate that the argument may repeat zero or more times.

In the following example, surrounding the matcher with $(...),+ will
match one or more expression, separated by commas. Also note that the semicolon is optional on the last case.
*/

mod repeat {
    // `find_min!` will calculate the minimum of any number of arguments.

    macro_rules! find_min {
        // Base case
        ($x:expr) => ($x);
        ($x:expr, $($y:expr),+) => {
            // Call `find_min!` on the tail `$y`
            std::cmp::min($x, find_min!($($y),+))
        }
    }

    pub fn main() {
        println!("{}", find_min!(1u32));
        println!("{}", find_min!(1u32 + 2, 2u32));
        println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
    }
}
