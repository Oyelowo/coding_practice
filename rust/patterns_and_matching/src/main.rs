fn main() {
    // @ Bindings
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
        // Message::Hello { id : idk} => println!("Found some other id: {}", idk),
    }
}

fn _main6() {
    // Extra Conditionals with Match Guards
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(10);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x= {:?}, y = {:?}", x, y);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn _main5() {
    // Ignoring Remaining Parts of a Value with ..
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3D { x: 0, y: 0, z: 0 };

    /*
    We list the x value and then just include the .. pattern. This is quicker than having to list y: _ and z: _, particularly when we’re working with structs that have lots of fields in situations where only one or two fields are relevant.
    */
    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }

    match origin {
        Point3D { x, y: _, z: _ } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => println!("Some numbers: {}, {}", first, last),
    }

    /*
    However, using .. must be unambiguous. If it is unclear which values are intended for matching and which should be ignored, Rust will give us an error. Listing 18-25 shows an example of using .. ambiguously, so it will not compile.
    */

    /*     match numbers {
        (.., second, ..) => println!("Some numbers: {}", second),
    } */
}

fn _main4() {
    // Ignoring an Unused Variable by Starting Its Name with _
    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));

    /*
    Note that there is a subtle difference between using only _ and using a name that starts with an underscore. The syntax _x still binds the value to the variable, whereas _ doesn’t bind at all. To show a case where this distinction matters, Listing 18-21 will provide us with an error.
    */
    /*   if let Some(_s) = s {
        println!("found a string");
    } */

    /*
    We’ll receive an error because the s value will still be moved into _s, which prevents us from using s again. However, using the underscore by itself doesn’t ever bind to the value. Listing 18-22 will compile without any errors because s doesn’t get moved into _.
    */
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}

fn _main3() {
    // Ignoring Parts of a Value with a Nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite existing value"),
        _ => {
            setting_value = new_setting_value;
        }
    };
    /* match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite existing value"),
        (None, Some(v)) => {
            setting_value = Some(v);
        }
        _ => println!(""),
    }; */

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth),
    }
}

fn _main2() {
    // Ignoring an Entire Value with _
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);
}

fn _main_destructuring_structs_tuples() {
    // Destructuring Structs and Tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

fn _main_destructuring_nested_structs_and_enums() {
    // Destructuring Nested Structs and Enums

    enum Color {
        Rgb(u8, u8, u8),
        Hsv(i32, i32, i32),
    };

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg2 = Message::Quit;

    let msg2 = Message::ChangeColor(Color::Rgb(3, 4, 255));

    match msg2 {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("color is red {} green {} blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    let msg = Message::Quit;

    let msg = Message::ChangeColor(Color::Rgb(3, 4, 255));

    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("the plane {} {}", x, y),
        Message::Write(text) => println!("printing the nice text: {}", text),
        Message::ChangeColor(color) => match color {
            Color::Rgb(r, g, b) => println!("color is red {} green {} blue {}", r, g, b),
            Color::Hsv(h, s, v) => println!("Anything"),
        },
    }
}

fn _main_destructuring_enums() {
    // Destructuring Enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(u8, u8, u8),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    let msg = Message::Quit;
    let msg = Message::Write(String::from("Some nice stuff"));
    let msg = Message::Move { x: 45, y: 67 };
    let msg = Message::Move { x: 0, y: 67 };
    let msg = Message::ChangeColor(0, 0, 0);

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x: 0, y } => {
            println!("on Y plane at {}", y);
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text Message: {}", text),
        Message::ChangeColor(0, 0, 0) => println!("Am I this black though?"),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {} green {} and blue {}", r, g, b)
        }
    }
}
struct Point {
    x: i32,
    y: i32,
}

fn _main_destructuring_structs() {
    // Destructuring Structs

    let p = Point { x: 55, y: 38 };

    let Point { x: a, y: b } = p;

    assert_eq!(55, a);
    assert_eq!(38, b);

    let Point { x, y } = p;

    assert_eq!(55, x);
    assert_eq!(38, y);

    let p2 = Point { x: 0, y: 5 };

    match p2 {
        Point { x: 0, y: 0 } => println!("At the origin"),
        Point { x, y: 0 } => println!("On the x axis at the {}", x),
        Point { x: 0, y } => println!("On the y axis at the {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    };
}

fn _main_matching_ranges() {
    // Matching ranges of value with ..=
    let x = 4;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("Something else"),
    }

    // Matching for char
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("Something else"),
    }
}

fn _main_multiple_patterns() {
    // Multiple Patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn _main_matching_named_variables() {
    // Matching named variables

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x= {:?}, y = {:?}", x, y);
}

fn _main_matching_literals() {
    let x = 4;

    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("three"),
        _ => println!("Anything"),
    }
}
