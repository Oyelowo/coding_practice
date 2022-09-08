use itertools::Itertools;

fn main() {
    let mut fn_ptr: fn(i32) -> i32 = add_one;
    // let mut fn_ptrx : Fn(i32) -> i32 = add_one;
    // let mut fn_ptrx : &dyn Fn(i32) -> i32 = &|x:i32| x + 1;
    // let mut fn_ptrx : Box<dyn Fn(i32) -> i32> = Box::new(|x:i32| x + 1);
    // let mut fn_ptrx : |x:i32| -> i32= |x:i32| x + 1;;
    // let mut fn_ptrx: |x: i32|  i32 = add_one;
    // let mut fn_ptr: fn(i32) -> i32 = add_one;

    assert_eq!(fn_ptr(3), 4);

    // capture-less closure cast to fn pointer
    fn_ptr = |x| x + 2;
    assert_eq!(fn_ptr(3), 5);

    // let p = fn_ptr(5);
    // println!("p : {p}");

    // Passing a regular function pointer in place of closure
    let nums = [1, 2, 3, 4, 5, -1, -434];
    let is_positive = |x: &i32| *x >= 0;
    let p = nums
        .map(add_one)
        .into_iter()
        .filter(is_positive)
        .sorted()
        .collect::<Vec<_>>();

    assert_eq!(p, [0, 2, 3, 4, 5, 6])
}

fn add_one(x: i32) -> i32 {
    x + 1
}
