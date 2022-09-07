use std::{array::IntoIter, iter::Map, rc::Rc, sync::Arc};

fn main() {
    let m: Vec<&dyn Animal> = vec![&Sheep, &Goat];
    let o: Vec<Box<dyn Animal>> = vec![Box::new(Sheep), Box::new(Goat)];
    let p: Vec<Rc<dyn Animal>> = vec![Rc::new(Sheep), Rc::new(Goat)];
    let x: Vec<Arc<dyn Animal>> = vec![Arc::new(Sheep), Arc::new(Goat)];
}

trait Animal {}

struct Sheep;

impl Animal for Sheep {}
struct Goat;

impl Animal for Goat {}

fn example() -> Box<dyn Animal> {
    if 5 > 4 {
        Box::new(Sheep)
    } else {
        Box::new(Goat)
    }
}

// fn example_generics_wont_work() -> impl Animal {
//     if 5 > 4 {
//         Box::new(Sheep)
//     } else {
//         Box::new(Goat)
//     }
// }

// type X = Box<dyn Map<IntoIter<i32>, Fn(i32) -> i32>>;
fn examplex(condition: bool, vec: Vec<i32>) -> Box<dyn Iterator<Item = i32>> {
    let iter = vec.into_iter();
    if condition {
        // Has type:
        // Box<Map<IntoIter<i32>, Fn(i32) -> i32>>
        // But is cast to:
        // Box<dyn Iterator<Item = i32>>
        Box::new(iter.map(|n| n * 2))
    } else {
        // Has type:
        // Box<Filter<IntoIter<i32>, Fn(&i32) -> bool>>
        // But is cast to:
        // Box<dyn Iterator<Item = i32>>
        Box::new(iter.filter(|&n| n >= 2))
    }
}
