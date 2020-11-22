fn main() {
    // Refutability Vs Irrefutability

    let some_option_value: Option<i8> = None;

    // Cant use an refutable pattern here in let statement
    // let Some(j) = some_option_value;

    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    /* // For this reason, match arms must use refutable patterns, except for the last arm, which should match any remaining values with an irrefutable pattern. 
    if let x = 5 {
        println!("{}", x);
    } */
}
