use core::cell::RefCell;
use std::rc::Rc;

fn main() {

}


#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}
