fn main() {
    let my_num = 20;

    if my_num != 0 {
        println!("run when not zero")
    }
    if my_num > 3 {
        println!("bigger than");
    } else {
        println!("Smaller than");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let is_big = 54 > 5;
    let size = if is_big { "Big" } else { "small" };
    println!("Big shit is {}", size);

    let mut counter = 0;
    let res = loop {
        counter += 1;

        if counter > 50 {
            break "result dope";
        }
        println!("again!");
    };

    println!("Give me my result, {}", res);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a: [u8; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (0..40).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
