use std::num::ParseIntError;

fn main() {
    let k = "errer";
    let p = k[0..3].parse::<i32>();
    // let p = k[0..3].parse::<i32>();
    println!("xxx: {p:?}");
    // assert_eq!(p, Err(ParseIntError));
}

fn func<T>(t: T)
where
    T: From<i32>,
    i32: Into<i32>,
{
    let p = T::from(4);
    let k: T = 4.into();
}

struct Lowo;

impl From<Dayo> for Lowo {
    fn from(_: Dayo) -> Self {
        todo!()
    }
}
struct Dayo;

fn funcxx<T>(t: T)
where
    T: From<Dayo>,
    Lowo: From<Dayo>,
    Dayo: Into<Lowo>,
{
    let m = T::from(Dayo);
    let p = Lowo::from(Dayo);
    // let k: T = 4.into();
}
