fn main() {
    let p = 4;
    p.bin
    let x = "s".to_string().chars().rev().collect::<String>();
    x.rev
    println!("Hello, world!");
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
         for (i, n1) in nums.iter().enumerate() {
             
             for (j, n2) in nums.iter().enumerate() {
                 
            let is_complement = ((n1 + n2) == target) && (i != j);
            if is_complement {
                return vec![i as i32, j as i32];
            }
        };
        };
        
        vec![]
    }
    
}