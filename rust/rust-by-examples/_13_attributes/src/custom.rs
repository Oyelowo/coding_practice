////////////////////////////////////////////////////////////////
// Custom cfg
/*
Some conditionals like target_os are implicitly provided by rustc,
but custom conditionals must be passed to rustc using the --cfg flag.
*/
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}

/*
Try to run this to see what happens without the custom cfg flag.

With the custom cfg flag:


$ rustc --cfg some_condition custom.rs && ./custom
condition met!
*/
