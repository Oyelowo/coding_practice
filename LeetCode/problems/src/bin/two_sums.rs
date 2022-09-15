struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, n1) in nums.iter().enumerate() {
            for (j, n2) in nums.iter().enumerate() {
                let is_complement = ((n1 + n2) == target) && (i != j);
                if is_complement {
                    return vec![i as i32, j as i32];
                }
            }
        }

        vec![]
    }
    pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, n1) in nums.iter().enumerate() {
            for (j, n2) in nums.iter().enumerate() {
                let is_complement = ((n1 + n2) == target) && (i != j);
                if is_complement {
                    return vec![i as i32, j as i32];
                }
            }
        }

        vec![]
    }
}

fn main() {}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut store = ::std::collections::HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let second_num = target - num;
        let second_num_index = store.get(&second_num);

        match second_num_index {
            Some(n) => {
                return vec![*n as i32, i as i32];
            }
            None => {
                store.insert(num, i as i32);
            }
        }
    }
    vec![]
}
