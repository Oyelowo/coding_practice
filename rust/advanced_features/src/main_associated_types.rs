// Advanced Traits
fn main() {
    
}

/*
The difference is that when using generics, as in Listing 19-13, we must annotate the types in each implementation; because we can also implement Iterator<String> for Counter or any other type, we could have multiple implementations of Iterator for Counter. In other words, when a trait has a generic parameter, it can be implemented for a type multiple times, changing the concrete types of the generic type parameters each time. When we use the next method on Counter, we would have to provide type annotations to indicate which implementation of Iterator we want to use.

With associated types, we don’t need to annotate types because we can’t implement a trait on a type multiple times. In Listing 19-12 with the definition that uses associated types, we can only choose what the type of Item will be once, because there can only be one impl Iterator for Counter. We don’t have to specify that we want an iterator of u32 values everywhere that we call next on Counter.
*/
// Using associated Types
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct TestM {
    name: String,
    age: u8,
}

impl Iterator for TestM {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(4)
    }
}

// Using Generics
pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}

struct TestM2 {
    name: String,
    age: u8,
}

impl Iterator2<u32> for TestM2 {
    fn next(&mut self) -> Option<u32> {
        Some(34)
    }
}
