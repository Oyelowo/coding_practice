pub fn two_number_sum(array: Vec<i64>, target_sum: i64) -> Vec<i64> {
    let k = array
        .iter()
        // iterate and filter out
        .filter(|&el| {
            !((el + el) == target_sum) && array.iter().any(|el2| (el + el2) == target_sum)
        })
        .map(|&m| m)
        .collect::<Vec<_>>();

    k
}

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
