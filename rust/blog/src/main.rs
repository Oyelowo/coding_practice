fn main() {
    let p = i32::default();
    // let  p = Default::default(i32);
    let u = println!("Hello, world! ---> {p}");
}

trait Superr {}

trait Lowo
where
    Self: Superr,
{
    type NumType: Eq + Ord + PartialEq + PartialOrd + Clone + Superr;

    fn get_another() -> i32;
    fn get_it() -> Self;
    fn borr(&self) -> Self;
    fn borr2(self: &Self) -> Self;
    fn borr_mut(&mut self) -> Self::NumType;
}

struct Nothing;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Moda;

impl Superr for Moda {}

impl Superr for Nothing {}

impl Lowo for Nothing {
    type NumType = Moda;

    fn get_another() -> i32 {
        todo!()
    }

    fn get_it() -> Self {
        todo!()
    }

    fn borr(&self) -> Self {
        todo!()
    }

    fn borr2(&self) -> Self {
        todo!()
    }

    fn borr_mut(&mut self) -> Self::NumType {
        todo!()
    }
}
