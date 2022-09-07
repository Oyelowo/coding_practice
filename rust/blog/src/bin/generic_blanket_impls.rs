use std::convert::TryInto;

fn main() {
    let p = 4i32.is_even();
}

trait Even {
    fn is_even(self) -> bool;
}

impl<T> Even for T
where
    T: std::ops::Rem<Output = T> + PartialEq<T> + Sized,
    u8: TryInto<T>,
    <u8 as TryInto<T>>::Error: std::fmt::Debug,
{
    fn is_even(self) -> bool {
        self % 2.try_into().unwrap() == 0.try_into().unwrap()
    }
}
