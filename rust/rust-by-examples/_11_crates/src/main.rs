// Using a Library
/*
To link a crate to this new library you may use rustc's --extern flag.
All of its items will then be imported under a module named the same as
the library. This module generally behaves the same way as any other module.
*/

// extern crate rary; // May be required for Rust 2015 edition or earlier

fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}
