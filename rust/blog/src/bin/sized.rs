fn main() {
    doma(&vec![9]);
    doma(&3);
}

struct Goat;

fn doma<T: ?Sized>(s: &T) {}
