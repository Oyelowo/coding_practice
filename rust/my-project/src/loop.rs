
fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    let kk = loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 100 {
            println!("OK, that's enough");

            // Exit this loop
            break "nice one";
        }
    };

    println!("Message from the loop {}", kk)
}
