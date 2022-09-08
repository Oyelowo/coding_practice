use std::{any::Any, ops::Deref};

fn main() {
    doma(&vec![9]);
    doma(&3);

    let vv: Vec<Box<dyn Any>> = vec![
        Box::new("x"),
        Box::new(1),
        Box::new("kjk".to_string()),
        Box::new(Goat),
    ];

    
    let k = vv.into_iter().map(doit).collect::<Vec<_>>();


    for p in k {
        // if p.downcast::<i32>().unwrap() == 2i32 {

        let p = p.downcast::<String>();
        println!("{p:?}");
        // }
    }
    // println!("kkk{k:?}");
}

struct Goat;

fn doit(mut any: Box<dyn Any>) -> Box<dyn Any> {
    if let Some(num) = any.downcast_mut::<i32>() {
        *num += 1;
    // } else if let Some(s) = any.downcast_ref::<String>() {
    //     println!("{s:?}");
    } else if let Some(x) = any.downcast_mut::<String>() {
        // *x += "string";
        x.push_str("opepep");
    } else if let Some(s) = any.downcast_ref::<String>() {
        println!("{s}");
    }
    any
}

fn doma<T: ?Sized>(s: &T) {}
