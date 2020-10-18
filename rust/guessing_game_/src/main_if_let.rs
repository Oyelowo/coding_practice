fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }

    let coin = Coin::Dime;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    count += 1;
    count += 1;
    println!("cccount {}", count);

    let coin2 = Coin::Dime;

    let mut count = 0;
    if let Coin::Quarter(state) = coin2 {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
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
