/*
format!: write formatted text to String
print!: same as format! but the text is printed to the console (io::stdout).
println!: same as print! but a newline is appended.
eprint!: same as format! but the text is printed to the standard error (io::stderr).
eprintln!: same as eprint!but a newline is appended.

*/

fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!(
        "{first_student}, this is {second_student}. {second_student}, this is {first_student}",
        first_student = "Alice",
        second_student = "Bob"
    );

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Special formatting can be specified after a `:`.
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // Special formatting can be specified after a `:`.
    println!("{} of {:0x} people know hexadecimal, the 6 don't", 1, 22);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:>16}", number = 1);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number = 1, width = 6);
    println!("{:04}", 1); // => "0042" with leading zeros
    let k = format!("{number:>0width$}", number = 1, width = 6);
    println!("{}erere", k);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    // println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"
    // FIXED
    println!("My name is {0}, {1} {0}", "Bond", "Oyelowo");

    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.
    // FIXED
    #[derive(Debug)]
    struct StructureFixed(i32);
    println!("This struct `{:?}` won't print...", StructureFixed(3));

    /*
        Add a println! macro that prints: Pi is roughly 3.142 by controlling the number
    of decimal places shown. For the purposes of this exercise, use let pi = 3.141592
    as an estimate for pi. (Hint: you may need to check the std::fmt documentation
        for setting the number of decimals to display)
        */
    let pi = 3.141592;
    println!("Pi is roughly {pi:.3}", pi = pi);
    println!("Pi is roughly {pi:.precision$}", pi = pi, precision = 3);
    println!("Pi is roughly {0:.1$}", pi, 3);
    println!("Pi is roughly {1:.*}", 3, pi);
    println!("Pi is roughly {:.*}", 3, pi);
}

/*
Activities
Fix the two issues in the above code (see FIXME) so that it runs without error.
Add a println! macro that prints: Pi is roughly 3.142 by controlling the number
of decimal places shown. For the purposes of this exercise, use let pi = 3.141592
as an estimate for pi. (Hint: you may need to check the std::fmt documentation
    for setting the number of decimals to display)

*/
