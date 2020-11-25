// Advanced Functions and Closures

fn main() {
    // Returning Closures

    // The following code tries to return a closure directly, but it won’t compile:
    /*     fn returns_closure() -> dyn Fn(i32) -> i32 {
        |x| x + 1
    } */

    // This code will compile just fine. For more about trait objects, refer to the section
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}

fn _main2() {
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    let answer = do_twice(add_one, 66);

    let answer2 = do_twice(|x| x + 1, 66);
    assert_eq!(answer, answer2);
    println!("a1 = a2 {} = {}", answer, answer2);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

type MyFunc = fn(i32) -> i32;

fn do_twice(f: MyFunc, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn _main1() {
    let answer = do_twice(add_one, 66);

    let answer2 = do_twice(|x| x + x, 66);
    assert_eq!(answer, answer2);
    println!("a1 = a2 {} = {}", answer, answer2);

    let list_of_numbers = vec![1, 2, 3];

    // As an example of where you could use either a closure defined inline or a named function, let’s look at a use of map. To use the map function to turn a vector of numbers into a vector of strings, we could use a closure, like this:
    let list_of_trings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    // Or we could name a function as the argument to map instead of the closure, like this:
    let list_of_trings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    let list_of_trings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| ToString::to_string(i))
        .collect();
}

enum Status {
    Value(u32),
    Stop,
}

trait Oye {
    fn fill(&self) -> Oye;
}

/*
trait Oye {
    fn fill(&self) -> Oye;
}
trait Oye {
    fn fill<T>(&self, t: T) -> T;
}
 */

struct Soft {
    components: Vec<Box<dyn Oye>>,
}
