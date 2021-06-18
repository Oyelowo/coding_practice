fn main() {
    used_function();
    cfg_stuff();
    conditional_function();
}

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
/*
Try to run this to see what happens without the custom cfg flag.

With the custom cfg flag:


$ rustc --cfg some_condition custom.rs && ./custom
condition met!
*/

////////////////////////////////////////////////////////////////
// cfg
/*
Configuration conditional checks are possible through two different operators:

the cfg attribute: #[cfg(...)] in attribute position
the cfg! macro: cfg!(...) in boolean expressions
While the former enables conditional compilation, the latter conditionally evaluates
to true or false literals allowing for checks at run-time. Both utilize identical argument syntax.
*/

// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn cfg_stuff() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}

////////////////////////////////////////////////////////////////

// Dead code
fn used_function() {}

// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// FIXME ^ Add an attribute to suppress the warning

/*
Crates
The crate_type attribute can be used to tell the compiler whether a crate is a binary or a library
(and even which type of library), and the crate_name attribute can be used to set the name of the crate.

However, it is important to note that both the crate_type and crate_name attributes have no effect
whatsoever when using Cargo, the Rust package manager. Since Cargo is used for the majority of Rust projects, this means real-world uses of crate_type and crate_name are relatively limited.


*/
// This crate is a library
// #![crate_type = "lib"]
// The library is named "rary"
// #![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

/* When the crate_type attribute is used, we no longer need to pass the --crate-type flag to rustc.


$ rustc lib.rs
$ ls lib*
library.rlib */
