use std::{path::Display, convert::TryInto};

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

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Option<Vec<i32>> {
        // let p = nums.iter().position(|n1| {

        // })
        for (i, n1) in nums.iter().enumerate() {
            let k = nums.iter().position(|&n2| (n1 + n2) == target);
            if let Some(n) = k {
                return Some(vec![i.try_into().unwrap(), n as i32]);
            }
        };
        None
    }
}

fn own_alien(alien: Alien) -> Alien {
    let a = Alien { id: 5 };
    let p = vec![1, 2];
    println!("early drop");
    // It will be dropped here before the function exits
    a
}
