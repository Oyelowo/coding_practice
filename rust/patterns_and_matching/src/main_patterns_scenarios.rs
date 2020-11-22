// All the Places Patterns Can Be Used

fn main() {
    // function parameters
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
}

fn main_let_statements() {
    let x = 5;
    let (x, y, z) = (1, 2, 3);
    let (x, _, z) = (1, 2, 3); // skipping
    let (x, ..) = (1, 2, 3); //skipping2
}

fn main_for_foreach() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index)
    }

    v.into_iter()
        .enumerate()
        .for_each(|(index, value)| println!("{} is at index {}", value, index))
}

fn main_while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn main_if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color,  {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
