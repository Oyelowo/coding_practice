use std::fmt::Display;

fn main() {
    {
        let r;

        {
            let x = 5;
            r = &x;
            println!("r: {}", r);
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    main2();
    main3();
    main4();
    main4();
}

fn main2() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn main3() {
    /*    Won't compile because string2 goes out of scope earlier
    let result;
      let string1 = String::from("long string is long");
      {
          let string2 = String::from("xyz");
          result = longest(string1.as_str(), string2.as_str());
      }
      println!("The longest string is {}", result); */
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/* fn longest2<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
   // result.as_str()
} */

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main4() {
    let novel = String::from("Call me Ishmael. Some years ago...");

    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let string1 = String::from("long string is longkekkrkrkrkrkrkrrrrrrrr");
    let jj = i.announce_and_return_part(string1.as_str());

    let s: &'static str = "I have a static lifetime";

    let kk = longest_with_an_announcement(string1.as_str(), novel.as_str(), "annn");
    println!("{}", kk);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
