fn main() {
    panic::main();
    option_and_unwrap::main();
    unpacking_with_question_marks::main();
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
