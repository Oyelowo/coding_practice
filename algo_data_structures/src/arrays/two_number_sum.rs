use itertools::Itertools;
use std::collections::HashMap;

pub fn two_number_sum1(array: Vec<i64>, target_sum: i64) -> Vec<i64> {
    let mut sum_tracker: HashMap<&i64, bool> = HashMap::new();

    for num in array.iter() {
        let potential_number = target_sum - num;
        if sum_tracker.contains_key(&potential_number) {
            return vec![potential_number, *num];
        }
        sum_tracker.insert(num, true);
    }
    return vec![];
}

pub fn two_number_sum2(array: Vec<i64>, target_sum: i64) -> Vec<i64> {
    let k = array
        .iter()
        .enumerate()
        // iterate and filter out
        .filter(|&(index1, el1)| {
            array
                .iter()
                .enumerate()
                .any(|(index2, el2)| index1 != index2 && (el1 + el2) == target_sum)
        })
        .map(|(_, el)| *el)
        .collect::<Vec<_>>();

    k
}

pub fn two_number_sum3(array: Vec<i64>, target_sum: i64) -> Vec<i64> {
    array
        .iter()
        // iterate and filter out
        .filter(|&el| {
            !((el + el) == target_sum) && array.iter().any(|el2| (el + el2) == target_sum)
        })
        .map(|&m| m)
        .collect::<Vec<_>>()
}

pub fn two_number_sum4(array: Vec<i64>, target_sum: i64) -> Vec<i64> {
    array.iter().fold(vec![], |mut acc, val| {
        /*    if (val * 2) == target_sum {
            return acc;
        }; */
        let potential_pair = array.iter().find(|&n| ((n + val) == target_sum));
        match potential_pair {
            None => acc,
            Some(pair) => match pair {
                bad_pair if bad_pair + bad_pair == target_sum => acc,
                _good_pair => {
                    acc.push(*val);
                    acc
                }
            },
        }
    })
}

pub fn two_number_sum(array: Vec<i64>, target_sum: i64) -> Vec<i64> {
    // let array = array.into_iter().sorted_by(|a, b| Ord::cmp(&a, &b)).collect();
    let array = array
        .into_iter()
        .sorted_by(|a, b| a.cmp(&b))
        .collect::<Vec<i64>>();

    let mut left = 0;
    let mut right = array.len() - 1;
    while left < right {
        let current_sum = array[left] + array[right];
         println!("current_sum{}", current_sum);
        println!("left{}", left);
        println!("right{}", right);
        println!("array[left]{}", array[left]);
        println!("array[right]{}", array[right]);
        println!("END");

        if current_sum == target_sum {
            return vec![array[left], array[right]];
        } else if current_sum < target_sum {
            left += 1;
        } else if current_sum > target_sum {
            right -= 1;
        }

       
    }

    return vec![];
}

/*
pub fn two_number_sum(array: Vec<i64>, target_sum: i64) -> Vec<i64> {
    array.iter().fold(vec![], |mut acc, val| {
        let num = match val * 2 {
            i if i == target_sum => None,
            _ => array.iter().find(|&n| n + val == target_sum),
        };

        // let num = array.iter().find(|&n| n + val == target_sum);

        println!("{:?}", val);
        println!("end");
        println!("{:?}", num);
        match num {
            None => acc,
            Some(num) => {
                acc.push(*num);
                acc
            }
        }
    })
}
*/
/*
x + y = target_sum

y = target_sum - x
*/

/*
export function twoNumberSum(array, targetSum) {
    // Write your code here.

    return array.filter((el, i) => array.some((el2, i2) => (i !== i2) && ((el2 + el) === targetSum)))
}

const arr = [3, 5, -4, 8, 11, 1, -1, 6];
const tar = 10;

console.log(twoNumberSum(arr, tar));


*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_number_sum() {
        let arr = vec![3, 5, -4, 8, 11, 1, -1, 6];
        let target_sum = 10;
        assert_eq!(two_number_sum(arr, target_sum), [11, -1]);
        let arr = vec![3, 5, -4, 8, 11, 1, -1, 6];
        let target_sum = 15;
        assert_eq!(two_number_sum(arr, target_sum), []);
        let arr = vec![4, 6];
        let target_sum = 10;
        assert_eq!(two_number_sum(arr, target_sum), [4, 6]);
        let arr = vec![4, 6, 1];
        let target_sum = 5;
        assert_eq!(two_number_sum(arr, target_sum), [4, 1]);
        let arr = vec![4, 6, 1, -3];
        let target_sum = 3;
        assert_eq!(two_number_sum(arr, target_sum), [6, -3]);
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let target_sum = 17;
        assert_eq!(two_number_sum(arr, target_sum), [8, 9]);
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 15];
        let target_sum = 18;
        assert_eq!(two_number_sum(arr, target_sum), [3, 15]);
        let arr = vec![-7, -5, -3, -1, 0, 1, 3, 5, 7];
        let target_sum = -5;
        assert_eq!(two_number_sum(arr, target_sum), [-5, 0]);
        let arr = vec![-21, 301, 12, 4, 65, 56, 210, 356, 9, -47];
        let target_sum = 163;
        assert_eq!(two_number_sum(arr, target_sum), [210, -47]);
        let arr = vec![-21, 301, 12, 4, 65, 56, 210, 356, 9, -47];
        let target_sum = 164;
        assert_eq!(two_number_sum(arr, target_sum), []);

        let arr = vec![3, 5, -4, 8, 11, 1, -1, 6];
        let target_sum = 15;
        assert_eq!(two_number_sum(arr, target_sum), []);

        let arr = vec![14];
        let target_sum = 15;
        assert_eq!(two_number_sum(arr, target_sum), []);

        let arr = vec![15];
        let target_sum = 15;
        assert_eq!(two_number_sum(arr, target_sum), []);
    }
}
