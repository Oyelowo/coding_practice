fn main_old() {
    let mut s = String::from("hello world");

    let word = first_word_bad(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    println!("{}", word)
}

fn first_word_bad(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("BBBBBB {}", b'd');
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main_old_() {
    let k: &str = "hhh";
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // error!

    println!("the first word is: {}", word);

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    let mut user1 = User {
        username: String::from("oyelowo"),
        email: String::from("oyelowo@gm.com"),
        sign_in_count: 23,
        active: true,
    };

    user1.email = String::from("");
    user1.username = String::from("fd");
    let kk = &mut user1;
    kk.active = false;
    kk.active = true;

    let user2: User = User {
        username: String::from("dayo"),
        email: String::from("dayo@k.co"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let r = black.0;
    let origin: Point = Point(0.0, 60.0, 60.0);
}

struct Color(u8, u8, u8);
struct Point(f32, f32, f32);

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 3434,
        active: false,
    }
}

/* struct UserA {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main2() {
    let user1 = UserA {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
} */

fn main() {
    let width1 = 30;
    let height1 = 30;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    let rect1 = (30, 40);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    let rect2: Rectangle = Rectangle {
        width: 53,
        height: 32,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );
    println!("Rect1 is {:#?}.", rect2);

    println!("Rect1 is area is {}.", rect2.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4? {:#?}", Rectangle::square(4).area());
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(side: u32) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
        }
    }
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
