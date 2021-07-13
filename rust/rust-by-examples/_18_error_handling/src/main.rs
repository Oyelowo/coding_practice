use rayon::prelude::*;
fn main() {
    panic::main();
    option_and_unwrap::main();
    unpacking_with_question_marks::main();
    combinators_map::main();
    combinators_and_then::main();
    result::main();
    map_for_result::main();
    multiple_error_types::main();
    wrapping_errors::main();
    iterating_over_results::main();
    println!("oylowo: {}", "oyelowo".len());
    println!("oylowo: {}", "oyelowo".chars().count());

    println!("öylowo: {}", "öyelowo".len());
    println!("öylowo: {}", "öyelowo".chars().count());
    println!("oylowo: {:?}", "öyelowo".chars());

    let a = vec!["a", "b", "c", "d", "e"];
    let b = ["a", "b"];
    coo(&b);
    coo(&a);

    let mut a = 12;
    let mut b = a;
    {
        let c = &mut a;
        b = 23;
        *c = 8;
        println!(" c: {:?}", c);
    }
    println!("a: {:?}, b: {:p}", a, &b);

    let p: MyStruct = "anything".into();
    println!("s: {:?}", p.s);

    let h = something(4).is_ok();

    let k = SpiderRobot {
        hardware_error_count: Cell::new(3),
        some_str: std::cell::RefCell::new("oyelowo".to_string()),
    };

    let c = k.hardware_error_count.get();
    let p = k.hardware_error_count.set(c + 5);
    k.some_str.borrow_mut().push_str("another");

    println!("1k{:?}", k);
    let mut k = SpiderRobot2 {
        hardware_error_count: 3,
        some_str: "oyelowo".into(),
    };
    let v = k.hardware_error_count;
    let m = k.hardware_error_count + 1;
    k.some_str.push_str("erere");
    println!("2k{:?}", k);
    println!("m{:?}", m);

    let g = vec![4];
    let g = std::cell::RefCell::new(g);
    g.borrow_mut().push(4);
    println!("{:?}", g);

    test_str(String::from("oyelowo"));
    let k: Year = 43.into();
    let k = Year::from(4);

    let mut k = RangeI32(0, 60, 5);
    println!("the main {}", k.next().unwrap());
    println!("the main {}", k.next().unwrap());
    println!("the main {}", k.next().unwrap());
    println!("the main {}", k.next().unwrap());

    /* for i in RangeI32(3, 6900000, 6) {
        println!("{}", i);
    } */
}
use std::borrow::Cow;
fn get_name() -> Cow<'static, str> {
    std::env::var("USER")
        .map(|v| Cow::Owned(v))
        .unwrap_or(Cow::Borrowed("whoever you are"))
}

fn get_name2() -> Cow<'static, str> {
    std::env::var("USER")
    .map(Cow::Owned)
    .unwrap_or_else(|_|Cow::Borrowed("whoever you are"))
}
fn get_name3() -> Cow<'static, str> {
    std::env::var("USER")
        .map(|v| v.into())
        .unwrap_or("whoever you are".into())
}




type Start = i32;
type End = i32;
type Step = i32;
struct RangeI32(Start, End, Step);

impl Display for RangeI32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Iterator for RangeI32 {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 >= self.1 {
            return None;
        }
        let result = self.0;
        self.0 += self.2;
        Some(result)
    }
}

fn test_year(n: i32) -> Year {
    n.into()
}

struct Year(i32);
impl From<i32> for Year {
    fn from(n: i32) -> Self {
        Self(n)
    }
}

struct S<T: ?Sized> {
    b: Box<T>,
}

fn test_str(strr: String) {
    println!("nothing")
}
use std::cell::{Cell, Ref};

#[derive(Debug)]
pub struct SpiderRobot {
    hardware_error_count: Cell<u32>,
    some_str: std::cell::RefCell<String>,
}

#[derive(Debug)]
pub struct SpiderRobot2 {
    hardware_error_count: u32,
    some_str: String,
}

fn test_from_cust() -> MyStruct {
    "new thing".into()
}

fn something(n: u32) -> Result<&'static str, &'static str> {
    if n > 3 {
        Ok("Greater than 3")
    } else {
        Err("ffd")
    }
}
struct MyStruct {
    s: String,
    name: String,
}

impl From<&'static str> for MyStruct {
    fn from(stri: &'static str) -> Self {
        Self {
            s: stri.to_string(),
            name: "Oyelowo".to_string(),
        }
    }
}

use std::fmt::{write, Debug};
use std::{
    error::Error,
    fmt::{self, Display, Write},
    io::{self, BufRead},
};
/// Read integers from a text file.
/// The file should have one number on each line.
fn read_numbers(
    file: &mut dyn BufRead,
) -> Result<Vec<i64>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;
        numbers.push(line.parse()?);
    }
    Ok(numbers)
}

fn coo(m: &[&str]) {
    println!("{}", m.join(","));
}
mod panic {
    /*
        panic
    The simplest error handling mechanism we will see is panic. It prints an error message,
    starts unwinding the stack, and usually exits the program. Here, we explicitly call panic on our error condition:
        */

    fn drink(beverage: &str) {
        if beverage == "juice" {
            panic!("Dont do that!")
        }

        println!("Some refreshing {} is all u need.", beverage);
    }

    pub fn main() {
        drink("water");
        // drink("juice");
    }
}

mod option_and_unwrap {
    /*
        Option & unwrap
    In the last example, we showed that we can induce program failure at will.
    We told our program to panic if we drink a sugary lemonade. But what if we
    expect some drink but don't receive one? That case would be just as bad, so it needs to be handled!

    We could test this against the null string ("") as we do with a lemonade.
    Since we're using Rust, let's instead have the compiler point out cases where there's no drink.

    An enum called Option<T> in the std library is used when absence is a possibility.
    It manifests itself as one of two "options":

    Some(T): An element of type T was found
    None: No element was found
    These cases can either be explicitly handled via match or implicitly with unwrap.
    Implicit handling will either return the inner element or panic.

    Note that it's possible to manually customize panic with expect,
    but unwrap otherwise leaves us with a less meaningful output than explicit handling.
    In the following example, explicit handling yields a more controlled result while
    retaining the option to panic if desired.
        */

    // The adult has seen it all, and can handle any drink well.
    // All drinks are handled explicitly using `match`.
    fn give_adult(drink: Option<&str>) {
        match drink {
            Some("lemonade") => println!("Too sugary"),
            Some(inner) => println!("Not that bad: {}", inner),
            None => println!("No drink! Go home boy!"),
        }
    }

    // Others will `panic` before drinking sugary drinks.
    // All drinks are handled implicitly using `unwrap`.
    fn drink(drink: Option<&str>) {
        // `unwrap` returns a `panic` when it receives a `None`.
        let inside = drink.unwrap();
        if inside == "lemonade" {
            panic!("No way! ")
        }
        println!("Too nice {}", inside);
    }

    pub fn main() {
        let water = Some("water");
        let lemonade = Some("lemonade");
        let void = None;

        give_adult(water);
        give_adult(lemonade);
        give_adult(void);

        let coffee = Some("coffee");
        // let nothing = None;

        drink(coffee);
        // drink(nothing);
    }
}

mod unpacking_with_question_marks {
    /*
    You can unpack Options by using match statements, but it's often easier to use the ? operator.
    If x is an Option, then evaluating x? will return the underlying value if x is Some,
    otherwise it will terminate whatever function is being executed and return None.
    */

    fn next_birthday(current_age: Option<u8>) -> Option<String> {
        // If `current_age` is `None`, this returns `None`.
        // If `current_age` is `Some`, the inner `u8` gets assigned to `next_age`
        let next_age = current_age?;
        Some(format!("Next year, you will be {}", next_age))
    }

    // You can chain many ?s together to make your code much more readable.

    struct Person {
        job: Option<Job>,
    }

    #[derive(Clone, Copy)]
    struct Job {
        phone_number: Option<PhoneNumber>,
    }

    #[derive(Clone, Copy)]
    struct PhoneNumber {
        area_code: Option<u8>,
        number: u32,
    }

    impl Person {
        // Gets the area code of the phone number of the person's job, if it exists.
        fn work_phone_area_code(&self) -> Option<u8> {
            // This would need many nested `match` statements without the `?` operator.
            // It would take a lot more code - try writing it yourself and see which
            // is easier.
            self.job?.phone_number?.area_code
        }

        fn work_phone_area_code_cumbersome(&self) -> Option<u8> {
            // This would need many nested `match` statements without the `?` operator.
            // It would take a lot more code - try writing it yourself and see which
            // is easier.
            match self.job {
                Some(j) => match j.phone_number {
                    Some(pn) => match pn.area_code {
                        Some(ac) => Some(ac),
                        None => None,
                    },
                    None => None,
                },
                None => None,
            }
        }

        fn work_phone_area_code_cumbersome2(&self) -> Option<u8> {
            // This would need many nested `match` statements without the `?` operator.
            // It would take a lot more code - try writing it yourself and see which
            // is easier.
            if let Some(j) = self.job {
                if let Some(pn) = j.phone_number {
                    if let Some(ac) = pn.area_code {
                        return Some(ac);
                    }
                }
            }
            return None;
        }
    }
    pub fn main() {
        let p = Person {
            job: Some(Job {
                phone_number: Some(PhoneNumber {
                    area_code: Some(44),
                    number: 000342,
                }),
            }),
        };

        assert_eq!(p.work_phone_area_code(), Some(44));
        assert_eq!(p.work_phone_area_code_cumbersome(), Some(44));
        assert_eq!(p.work_phone_area_code_cumbersome2(), Some(44));
    }
}

mod combinators_map {
    /*
        match is a valid method for handling Options.
        However, you may eventually find heavy usage tedious,
        especially with operations only valid with an input. In these cases,
        combinators can be used to manage control flow in a modular fashion.

    Option has a built in method called map(), a combinator
    for the simple mapping of Some -> Some and None -> None.
    Multiple map() calls can be chained together for even more flexibility.

    In the following example, process() replaces all functions previous to it while staying compact.
        */

    #![allow(dead_code)]

    #[derive(Debug)]
    enum Food {
        Apple,
        Carrot,
        Potato,
    }

    #[derive(Debug)]
    struct Peeled(Food);

    #[derive(Debug)]
    struct Chopped(Food);

    #[derive(Debug)]
    struct Cooked(Food);

    // Peeling food. If there isn't any, then return `None`.
    // Otherwise, return the peeled food.
    fn peel(food: Option<Food>) -> Option<Peeled> {
        match food {
            Some(food) => Some(Peeled(food)),
            None => None,
        }
    }

    // Chopping food. If there isn't any, then return `None`.
    // Otherwise, return the chopped food.
    fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
        match peeled {
            Some(Peeled(food)) => Some(Chopped(food)),
            None => None,
        }
    }

    fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
        chopped.map(|Chopped(food)| Cooked(food))
        //1.  Longer version using destructuring
        /*         chopped.map(|(food)| {
            let Chopped(food) = food;
            Cooked(food)
        }) */
        //2. Longer version using inner access of the New type
        // chopped.map(|food| Cooked(food.0))
        // 3. Using ?
        // Some(Cooked(chopped?.0))
    }

    // A function to peel, chop, and cook food all in sequence.
    // We chain multiple uses of `map()` to simplify the code.
    fn process(food: Option<Food>) -> Option<Cooked> {
        // let k = food.map(|food| Peeled(food));
        /*         let k = Some(Peeled(food?))
        .map(|food| Chopped(food.0))
        .map(|food| Cooked(food.0)); */
        /*       let k = food
        .map(|f| Peeled(f))
        .map(|peeled_food| Chopped(peeled_food.0))
        .map(|chopped_food| Cooked(chopped_food.0)); */
        let k = food
            .map(|f| Peeled(f))
            .map(|Peeled(food)| Chopped(food))
            .map(|Chopped(food)| Cooked(food));
        k
    }

    // Check whether there's food or not before trying to eat it!
    fn eat(food: Option<Cooked>) {
        // let p = food?.0;
        match food {
            Some(cooked_food) => println!("Let's eat some {:?}!", cooked_food),
            None => println!("Not cooked, tastes raw!"),
        }
    }

    pub fn main() {
        use Food::*;
        // let apple = Some(Food::Apple);
        let apple = Some(Apple);
        let carrot = Some(Carrot);
        let potato: Option<Food> = None;

        let cooked_apple = cook(chop(peel(apple)));
        let cooked_carrot = cook(chop(peel(carrot)));
        // Let's try the simpler looking `process()` now.
        let cooked_potato = process(potato);

        eat(cooked_apple);
        eat(cooked_carrot);
        eat(cooked_potato);
    }
}

mod combinators_and_then {
    /*
    map() was described as a chainable way to simplify match statements.
    However, using map() on a function that returns an Option<T> results in the nested Option<Option<T>>.
    Chaining multiple calls together can then become confusing.
    That's where another combinator called and_then(), known in some languages as flatmap, comes in.

    and_then() calls its function input with the wrapped value and returns the result.
    If the Option is None, then it returns None instead.

    In the following example, cookable_v2() results in an Option<Food>.
    Using map() instead of and_then() would have given an Option<Option<Food>>,
    which is an invalid type for eat().
    */

    #![allow(dead_code)]

    #[derive(Debug)]
    enum Food {
        CordonBleu,
        Steak,
        Sushi,
    }
    #[derive(Debug)]
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
    }

    // We don't have the ingredients to make Sushi.
    fn have_ingredients(food: Food) -> Option<Food> {
        match food {
            Food::Sushi => None,
            _ => Some(food),
        }
    }

    // We have the recipe for everything except Cordon Bleu.
    fn have_recipe(food: Food) -> Option<Food> {
        match food {
            Food::CordonBleu => None,
            _ => Some(food),
        }
    }

    // To make a dish, we need both the recipe and the ingredients.
    // We can represent the logic with a chain of `match`es:
    fn cookable_v1(food: Food) -> Option<Food> {
        match have_recipe(food) {
            None => None,
            Some(food) => match have_ingredients(food) {
                None => None,
                Some(food) => Some(food),
            },
        }
    }

    fn cookable_v2_with_and_then(food: Food) -> Option<Food> {
        // Returns Option<Food>
        let k = have_ingredients(food).and_then(have_recipe);
        k
        // Using map to do the same but inconvenient
        // returns Option<Option<Food>>
        /*  let k = have_ingredients(food).map(|f|have_recipe(f));
        Some(k??) */
    }
    fn cookable_v2_with_map(food: Food) -> Option<Food> {
        // Returns Option<Food>
        /*     let k = have_ingredients(food).and_then(have_recipe);
        k */
        // Using map to do the same but inconvenient
        // returns Option<Option<Food>>
        let k = have_ingredients(food).map(|f| have_recipe(f));
        Some(k??)
    }

    fn eat(food: Food, day: Day) {
        match cookable_v2_with_and_then(food) {
            Some(foodie) => println!("Yay! On {:?} we get to eat {:?}.", day, foodie),
            None => println!("Oh no. We don't get to eat on {:?}?", day),
        }
    }

    pub fn main() {
        let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);
        eat(cordon_bleu, Day::Monday);
        eat(steak, Day::Tuesday);
        eat(sushi, Day::Wednesday);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Result
/*
Result is a richer version of the Option type that describes possible error instead of possible absence.

That is, Result<T, E> could have one of two outcomes:

Ok(T): An element T was found
Err(E): An error was found with element E
By convention, the expected outcome is Ok while the unexpected outcome is Err.

Like Option, Result has many methods associated with it. unwrap(), for example,
either yields the element T or panics. For case handling,
there are many combinators between Result and Option that overlap.

In working with Rust, you will likely encounter methods that return the Result type,
such as the parse() method. It might not always be possible to parse a
string into the other type, so parse() returns a Result indicating possible failure.

Let's see what happens when we successfully and unsuccessfully parse() a string:
*/

mod result {
    fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
        // Let's try using `unwrap()` to get the number out. Will it bite us?
        let first_number: i32 = first_number_str.parse().unwrap();
        let second_number = second_number_str.parse::<i32>().unwrap();
        first_number * second_number
    }

    pub fn main() {
        without_result();
        main_with_result();
        main_with_result2();
    }

    pub fn without_result() {
        let twenty = multiply("10", "2");
        println!("double is {}", twenty);

        // let tt = multiply("t", "2");
        // println!("double is {}", tt);
    }

    /*
        In the unsuccessful case, parse() leaves us with an error for unwrap() to panic on.
        Additionally, the panic exits our program and provides an unpleasant error message.

    To improve the quality of our error message, we should be more specific about
    the return type and consider explicitly handling the error.
        */

    /*
            Using Result in main
    The Result type can also be the return type of the main function if specified explicitly.
    Typically the main function will be of the form:
            */
    use std::num::ParseIntError;

    pub fn main_with_result() -> Result<(), ParseIntError> {
        let number_str = "10";
        let number = match number_str.parse::<i32>() {
            Ok(n) => n,
            Err(err) => return Err(err),
        };

        println!("{}", number);
        Ok(())
    }

    pub fn main_with_result2() -> Result<(), ParseIntError> {
        let number_str = "10";
        let number = number_str.parse::<i32>()?;

        println!("{}", number);
        Ok(())
    }
}

mod map_for_result {
    use std::num::ParseIntError;

    /*
        map for Result
    Panicking in the previous example's multiply does not make for robust code.
    Generally, we want to return the error to the caller so it can decide what is the right way to respond to errors.

    We first need to know what kind of error type we are dealing with.
    To determine the Err type, we look to parse(), which is implemented
     with the FromStr trait for i32. As a result, the Err type is specified as ParseIntError.

    In the example below, the straightforward match statement leads to code that is overall more cumbersome.
        */

    /*
            aliases for Result
    How about when we want to reuse a specific Result type many times? Recall that Rust allows us to create aliases. Conveniently, we can define one for the specific Result in question.

    At a module level, creating aliases can be particularly helpful. Errors found in a specific module often have the same Err type, so a single alias can succinctly define all associated Results. This is so useful that the std library even supplies one: io::Result!

    Here's a quick example to show off the syntax:
            */
    type AliasedResult<T> = Result<T, ParseIntError>;
    // With the return type rewritten, we use pattern matching without `unwrap()`.
    fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
        match first_number_str.parse::<i32>() {
            Ok(first_number) => match second_number_str.parse::<i32>() {
                Ok(second_number) => Ok(first_number * second_number),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    fn print(result: AliasedResult<i32>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    /*
    Luckily, Option's map, and_then, and many other combinators are also implemented for Result. Result contains a complete listing.
     */

    // As with `Option`, we can use combinators such as `map()`.
    // This function is otherwise identical to the one above and reads:
    // Modify n if the value is valid, otherwise pass on the error.
    fn multiply2(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
        let k = first_number_str
            .parse::<i32>()
            .and_then(|a| second_number_str.parse::<i32>().map(|b| a * b));

        let k = first_number_str
            .parse::<i32>()
            .and_then(|a| second_number_str.parse::<i32>().and_then(|b| Ok(b * a)));
        k
        /*      let k = first_number_str.parse::<i32>()? * second_number_str.parse::<i32>()?;
        Ok(k) */
    }

    pub fn main() {
        // This still presents a reasonable answer.
        let twenty = multiply("10", "2");
        print(twenty);

        // The following now provides a much more helpful error message.
        let tt = multiply("t", "2");
        print(tt);

        // This still presents a reasonable answer.
        let twenty = multiply2("10", "2");
        print(twenty);

        // The following now provides a much more helpful error message.
        let tt = multiply2("t", "2");
        print(tt);
    }

    mod early_returns {
        /*
                In the previous example, we explicitly handled the errors using combinators.
                Another way to deal with this case analysis is to use a combination of match statements and early returns.

        That is, we can simply stop executing the function and return the error if one occurs.
        For some, this form of code can be easier to both read and write.
        Consider this version of the previous example, rewritten using early returns:
                */
        use std::num::ParseIntError;

        fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
            let first_number = match first_number_str.parse::<i32>() {
                Ok(first_number) => first_number,
                Err(e) => return Err(e),
            };

            let second_number = match second_number_str.parse::<i32>() {
                Ok(second_number) => second_number,
                Err(e) => return Err(e),
            };

            Ok(first_number * second_number)
        }

        fn print(result: Result<i32, ParseIntError>) {
            match result {
                Ok(n) => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        fn main() {
            print(multiply("10", "2"));
            print(multiply("t", "2"));
        }

        /*
                At this point, we've learned to explicitly handle errors using combinators
                nd early returns. While we generally want to avoid panicking,
                explicitly handling all of our errors is cumbersome.

        In the next section, we'll introduce ? for the cases where we simply
        need to unwrap without possibly inducing panic.
                */
    }
}

mod question_mark_error {
    /*
        Introducing ?
    Sometimes we just want the simplicity of unwrap without the possibility of a panic.
    Until now, unwrap has forced us to nest deeper and deeper when what we really
    wanted was to get the variable out. This is exactly the purpose of ?.

    Upon finding an Err, there are two valid actions to take:

    panic! which we already decided to try to avoid if possible
    return because an Err means it cannot be handled
    ? is almost1 exactly equivalent to an unwrap which returns instead of panicking on Errs.
    Let's see how we can simplify the earlier example that used combinators:
        */

    use std::num::ParseIntError;

    fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        let first_number = first_number_str.parse::<i32>()?;
        let second_number = second_number_str.parse::<i32>()?;

        Ok(first_number * second_number)
    }

    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn main() {
        print(multiply("10", "2"));
        print(multiply("t", "2"));
    }

    /*
        The try! macro
    Before there was ?, the same functionality was achieved with the try! macro.
    The ? operator is now recommended, but you may still find try! when looking at older code.
    The same multiply function from the previous example would look like this using try!:
        */
    /*       fn multiply_try(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        let first_number = try!(first_number_str.parse::<i32>());
        let second_number = try!(second_number_str.parse::<i32>());

        Ok(first_number * second_number)
    }
        */
}

mod multiple_error_types {
    use std::{error::Error, num::ParseIntError};

    /*
        The previous examples have always been very convenient;
        Results interact with other Results and Options interact with other Options.

    Sometimes an Option needs to interact with a Result, or a Result<T, Error1>
    needs to interact with a Result<T, Error2>. In those cases, we want to manage our
    different error types in a way that makes them composable and easy to interact with.

    In the following code, two instances of unwrap generate different error types.
    Vec::first returns an Option, while parse::<i32> returns a Result<i32, ParseIntError>:
        */
    fn double_first1(vec: Vec<&str>) -> i32 {
        let first = vec.first().unwrap(); // Generate error 1
        2 * first.parse::<i32>().unwrap() // Generate error 2
    }

    /*
        Pulling Results out of Options
    The most basic way of handling mixed error types is to just embed them in each other.
        */

    fn double_first2(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
        vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
    }

    /*
    There are times when we'll want to stop processing on errors (like with ?) but
    keep going when the Option is None. A couple of combinators come in handy to swap the Result and Option.
    */

    fn double_first3(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
        let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));
        opt.map_or(Ok(None), |r| r.map(|e| Some(e)))
    }

    /*    fn double_first3(vec: Vec<&str>) -> Result<i32, impl Error> {
        let first = vec.first()?; // Generate error 1
        let k = 2 * first.parse::<i32>()?; // Generate error 2
        k
    } */

    pub fn main() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];
        let k = numbers.first();

        println!("The first doubled is {:?}", double_first1(numbers));

        println!("The first doubled is {:?}", double_first2(empty));
        // Error 1: the input vector is empty

        println!("The first doubled is {:?}", double_first3(strings));
        // Error 2: the element doesn't parse to a number
    }
}

mod define_error_type {
    /*
        Defining an error type
    Sometimes it simplifies the code to mask all of the different
    errors with a single type of error. We'll show this with a custom error.

    Rust allows us to define our own error types. In general, a "good" error type:

    Represents different errors with the same type
    Presents nice error messages to the user
    Is easy to compare with other types
    Good: Err(EmptyVec)
    Bad: Err("Please use a vector with at least one element".to_owned())
    Can hold information about the error
    Good: Err(BadChar(c, position))
    Bad: Err("+ cannot be used here".to_owned())
    Composes well with other errors
        */

    use std::fmt;

    // Define our error types. These may be customized for our error handling cases.
    // Now we will be able to write our own errors, defer to an underlying error
    // implementation, or do something in between.
    #[derive(Debug, Clone)]
    struct DoubleError;

    type Result<T> = std::result::Result<T, DoubleError>;

    // Generation of an error is completely separate from how it is displayed.
    // There's no need to be concerned about cluttering complex logic with the display style.
    //
    // Note that we don't store any extra info about the errors. This means we can't state
    // which string failed to parse without modifying our types to carry that information.
    impl fmt::Display for DoubleError {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            write!(fmt, "Invalid first item to double")
        }
    }

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        let k = vec
            .first()
            // Change the error to our new type.
            .ok_or(DoubleError)
            .and_then(|s| s.parse::<i32>().map_err(|_| DoubleError))
            .map(|i| 2 * i);
        k
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("the first double is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }
}

mod boxing_errors {
    /*
        Boxing errors
    A way to write simple code while preserving the original errors is to Box them.
    The drawback is that the underlying error type is only known at runtime and not statically determined.

    The stdlib helps in boxing our errors by having Box implement conversion from any
    type that implements the Error trait into the trait object Box<Error>, via From.
        */

    use std::error;
    use std::fmt;

    #[derive(Debug, Clone)]
    struct EmptyVec;

    // Change the alias to `Box<error::Error>`.
    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    impl fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    impl error::Error for EmptyVec {}

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        let k = vec
            .first()
            //
            .ok_or_else(|| EmptyVec.into())
            //
            .and_then(|s| {
                s.parse::<i32>()
                    //
                    .map_err(|e| e.into())
                    .map(|i| i * 2)
            });
        k
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    /*
        Other uses of ?
    Notice in the previous example that our immediate reaction to
    calling parse is to map the error from a library error into a boxed error:

    .and_then(|s| s.parse::<i32>()
        .map_err(|e| e.into())
    Since this is a simple and common operation, it would be convenient
    if it could be elided. Alas, because and_then is not sufficiently flexible,
    it cannot. However, we can instead use ?.

    ? was previously explained as either unwrap or return Err(err).
    This is only mostly true. It actually means unwrap or return Err(From::from(err)).
    Since From::from is a conversion utility between different types,
    this means that if you ? where the error is convertible to the return type, it will convert automatically.

    Here, we rewrite the previous example using ?. As a result,
    the map_err will go away when From::from is implemented for our error type:
        */

    fn double_first2(vec: Vec<&str>) -> Result<i32> {
        // let k = vec.first().ok_or(EmptyVec)?.parse::<i32>()?;
        let first = vec.first().ok_or(EmptyVec)?;
        let parsed = first.parse::<i32>()?;

        Ok(parsed * 2)
    }

    /*
    This is actually fairly clean now. Compared with the original panic,
    it is very similar to replacing the unwrap calls with ? except that
    the return types are Result. As a result, they must be destructured at the top level.
    */

    fn main() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}

mod wrapping_errors {
    /*
    An alternative to boxing errors is to wrap them in your own error type.
     */
    use std::error;
    use std::error::Error as _;
    use std::fmt;
    use std::num::ParseIntError;

    type ResultWithDoubleError<T> = std::result::Result<T, DoubleError>;

    #[derive(Debug)]
    enum DoubleError {
        EmptyVec,
        // We will defer to the parse error implementation for their error.
        // Supplying extra info requires adding more data to the type.
        Parse(ParseIntError),
    }

    impl fmt::Display for DoubleError {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                DoubleError::EmptyVec => {
                    write!(fmt, "please use a vector with at least one element")
                }
                // The wrapped error contains additional information and is available
                // via the source() method.
                DoubleError::Parse(..) => {
                    write!(fmt, "the provided string could not be parsed as int")
                }
            }
        }
    }

    impl error::Error for DoubleError {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match *self {
                DoubleError::EmptyVec => None,
                // The cause is the underlying implementation error type. Is implicitly
                // cast to the trait object `&error::Error`. This works because the
                // underlying type already implements the `Error` trait.
                DoubleError::Parse(ref e) => Some(e),
            }
        }
    }
    // Implement the conversion from `ParseIntError` to `DoubleError`.
    // This will be automatically called by `?` if a `ParseIntError`
    // needs to be converted into a `DoubleError`.
    impl From<ParseIntError> for DoubleError {
        fn from(err: ParseIntError) -> DoubleError {
            DoubleError::Parse(err)
        }
    }

    fn double_first(vec: Vec<&str>) -> ResultWithDoubleError<i32> {
        let first = vec.first().ok_or(DoubleError::EmptyVec)?;
        // Here we implicitly use the `ParseIntError` implementation of `From` (which
        // we defined above) in order to create a `DoubleError`.
        let parsed = first.parse::<i32>()?;
        Ok(2 * parsed)
    }

    fn print(result: ResultWithDoubleError<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => {
                println!("Error: {}", e);
                if let Some(source) = e.source() {
                    println!("  Caused by: {}", source);
                }
            }
        }
    }

    /*
    This adds a bit more boilerplate for handling errors and might not be needed in all applications.
    There are some libraries that can take care of the boilerplate for you.
    */

    pub fn main() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}

mod iterating_over_results {
    /*
    An Iter::map operation might fail, for example:
     */

    fn iter_map() {
        let strings = vec!["tofu", "93", "18"];
        let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
        println!("Results: {:?}", numbers);
    }
    /*
        Let's step through strategies for handling this.

        Ignore the failed items with filter_map()
    filter_map calls a function and filters out the results that are None.
        */

    fn filter_map() {
        let strings = vec!["tofu", "93", "18"];
        let numbers: Vec<_> = strings
            .into_iter()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        println!("Results: {:?}", numbers);
    }

    /*
        Fail the entire operation with collect()
    Result implements FromIter so that a vector of results (Vec<Result<T, E>>)
    can be turned into a result with a vector (Result<Vec<T>, E>).
    Once an Result::Err is found, the iteration will terminate.
        */

    fn fail_all() {
        let strings = vec!["tofu", "93", "18"];
        let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
        println!("Results: {:?}", numbers);
        /*
        This same technique can be used with Option.
        */
    }

    /*
    Collect all valid values and failures with partition()
    */

    fn partition_valid_values_n_failures() {
        let strings = vec!["tofu", "93", "18"];
        let (numbers, errors): (Vec<_>, Vec<_>) = strings
            .into_iter()
            .map(|s| s.parse::<i32>())
            .partition(Result::is_ok);
        println!("Numbers: {:?}", numbers);
        println!("Errors: {:?}", errors);
    }

    /*
    When you look at the results, you'll note that everything is still
    wrapped in Result. A little more boilerplate is needed for this.
    */

    fn partition_more() {
        let strings = vec!["tofu", "93", "18"];
        let (numbers, errors): (Vec<_>, Vec<_>) = strings
            .into_iter()
            .map(|s| s.parse::<i32>())
            .partition(Result::is_ok);
        let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
        let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
        println!("Numbers: {:?}", numbers);
        println!("Errors: {:?}", errors);
    }

    pub fn main() {
        iter_map();
        filter_map();
        fail_all();
        partition_valid_values_n_failures();
        partition_more();
    }
}
