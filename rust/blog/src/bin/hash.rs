use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

fn main() {
    let mut p = HashSet::new();
    p.insert(Pointt { x: 3, y: 3 });
    
    
    let mut p = HashSet::new();
    p.insert(Point { x: 2, y: 2 });
}

/* trait Hash
where
    Self: Sized,
{
    fn hash<H: Hasher>(&self, state: &mut H);

    // provided default impls
    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H);
} */

#[derive(Hash, PartialEq, Eq)]
struct Pointt {
    x: i32,
    y: i32,
}
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.x);
        state.write_i32(self.y);
        // self.x.hash(state);
        // self.y.hash(state);
    }
}
