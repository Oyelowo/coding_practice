mod arrays;

use arrays as ar;

fn main() {
    println!("Hello, world!");
    let arr = vec![3, 5, -4, 8, 11, 1, -1, 6];
    let target_sum = 10;

    let arr = vec![3, 5, -4, 8, 11, 1, -1, 6];
    let target_sum = 15;

    let arr = vec![4, 6];
    let target_sum = 10;

    let arr = vec![4, 6, 1];
    let target_sum = 5;

    let arr = vec![4, 6, 1, -3];
    let target_sum = 3;

    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let target_sum = 17;

    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 15];
    let target_sum = 18;

    let arr = vec![-7, -5, -3, -1, 0, 1, 3, 5, 7];
    let target_sum = -5;

    let arr = vec![-21, 301, 12, 4, 65, 56, 210, 356, 9, -47];
    let target_sum = 163;

    let arr = vec![-21, 301, 12, 4, 65, 56, 210, 356, 9, -47];
    let target_sum = 164;

    println!("{:?}", ar::two_number_sum::two_number_sum(arr, target_sum));


}



fn main2() {
    let result = (0..10).flat_map(|_| {
       let vec: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
       vec.into_iter()
    }).collect::<Vec<_>>();
}
