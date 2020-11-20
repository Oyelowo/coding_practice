use std::ops::Deref;

use crate::List::{Cons, Nil};


fn main() {
    let x = 5;
    // let y = &x;
    // let y = Box::new(x);
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T)-> MyBox<T> {
        Self(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn cons_stuff() {
    let b: Box<i128> = Box::new(5);
    println!("b = {}", b);
    println!("fib = {}", fib(5));

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}


fn fib(n: i32) -> i32 {
    if n < 1 {
        return 1;
    }

    fib(n-2) + fib(n-1)
}

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
