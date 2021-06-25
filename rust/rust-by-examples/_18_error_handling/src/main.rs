fn main() {
    panic::main();
    option_and_unwrap::main();
    unpacking_with_question_marks::main();
    combinators_map::main();
    combinators_and_then::main();
    result::main();
    map_for_result::main();
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

    // With the return type rewritten, we use pattern matching without `unwrap()`.
    fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        match first_number_str.parse::<i32>() {
            Ok(first_number) => match second_number_str.parse::<i32>() {
                Ok(second_number) => Ok(first_number * second_number),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    fn print(result: Result<i32, ParseIntError>) {
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
    fn multiply2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
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
}
