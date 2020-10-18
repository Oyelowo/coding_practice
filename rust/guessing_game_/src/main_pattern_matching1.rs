fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);
}

/*
Invalid: has to handle all cases
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
} */
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn print_test() {
    value_in_cents(Coin::Quarter(UsState::Alabama));
    value_in_cents(Coin::Quarter(UsState::Alaska));
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Penny);
}

fn the_placeholder() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("one"),
        3 => println!("one"),
        5 => println!("one"),
        7 => println!("one"),
        _ => (),
    }
}
