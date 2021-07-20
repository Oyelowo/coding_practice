fn main() {
    println!("Hello, world!");
    let mine = vec_custom![1, 2, 3];
    let mine2 = vec_custom2![43, 56];

    let k = Some(5).ok_or_else(|| 5).expect("r");


    println!("rere{:?}", mine);
}

#[macro_export]
macro_rules! vec_custom {
    ($($x:expr),*) => {
        {
            let mut vv = Vec::new();
            $(
                vv.push($x);
            )*
            vv
        }
    };
}

#[macro_export]
macro_rules! vec_custom2 {
    ($($x:expr),*) => {
        {
            <[_]>::into_vec(Box::new([$($x),*]))
        }
    };
}
