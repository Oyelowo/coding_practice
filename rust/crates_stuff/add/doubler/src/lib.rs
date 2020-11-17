#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use add_one;
pub fn double_up(x: i32) -> i32 {
    add_one::add_one(x) + add_one::add_one(x)
}