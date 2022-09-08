use std::ops::Deref;

fn main() {
    let a = Lowo;
    let p = &*a;
    let m = xx(&a);
    let m = xx(&Lowo);
    let d = Dayo;
    let m = xx(&d);
}

fn xx(a: &Lowo) {
    // let x = *a;
}

struct Lowo;
struct Dayo;

impl Deref for Lowo {
    type Target=Dayo;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}
impl Deref for Dayo {
    type Target=Lowo;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

