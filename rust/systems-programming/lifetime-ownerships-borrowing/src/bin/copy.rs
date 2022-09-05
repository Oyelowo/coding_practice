use std::path::Display;

#[derive(Debug, Clone, Copy)]
struct Alien {
    id: u128,
}

// impl Drop for Alien {
//     fn drop(&mut self) {
//         println!("drops, {self:?}");
//         // todo!()
//     }
// }

// impl std::fmt::Display for Alien {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(self.id.to_string().as_str())
//     }
// }

fn main() {
    let alien1 = Alien { id: 1 };
    let alien2 = Alien { id: 2 };

    own_alien(alien1);
    println!("alien1: {alien1:?}");
    // println!("alien1: {alien1:?}");
    // println!("alien1: {alien1}");
}

fn own_alien(alien: Alien) -> Alien{
    let a = Alien{ id: 5};
    println!("early drop");
    // It will be dropped here before the function exits
    a
}
