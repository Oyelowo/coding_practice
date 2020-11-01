
use adder;

mod common;

#[test]
fn it_adds_two() {
    assert_eq!(3, adder::add_two(2));
}