fn main() {
    let x = vec![1,2,3 ];

    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x);


    let y = vec![1,2,3];

    assert!(equal_to_x(y));
}

fn main1() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 5;

    assert!(equal_to_x(y));
}
