use std::cell::RefCell;

trait Animal {
    fn live(&self);
}

struct Goat {}

impl Animal for Goat {
    fn live(&self) {
        todo!()
    }
}

struct Human {}

impl Animal for Human {
    fn live(&self) {
        todo!()
    }
}

fn main() {
    let k: Box<dyn Animal> = Box::new(Goat {});
    let mut k: Box<dyn Animal> = Box::new(Goat {});

    k = Box::new(Human {});

    let mut k: &dyn Animal = &Goat {};

    k = &Human {};

    let c = 5;
    let m: Box<dyn Animal> = if c > 6 {
        Box::new(Goat {})
    } else {
        Box::new(Human {})
    };
    let c = 5;
    let m: &dyn Animal = if c > 6 { &Goat {} } else { &Human {} };
}
