#[macro_export]
macro_rules! vecc {
    ( $( $x:expr ), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}


