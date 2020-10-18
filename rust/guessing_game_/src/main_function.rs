fn main() {
    println!("Hello from the other side");
    let real_number = another_function(45.0, 56.01);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    println!("My real number, {}", real_number);
}

fn another_function(x: f32, y: f32) -> f32 {
    println!("The coordinate {}, {},", x, y);

    34.0 + x + y
}
