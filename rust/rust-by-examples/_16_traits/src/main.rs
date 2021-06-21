fn main() {
    derive::main();
    returning_traits_with_dyn::main_dyn();
    operator_overloading::main();
    drop::main();
    iterators::main();
}

mod iterators {
    /*
        The Iterator trait is used to implement iterators over collections such as arrays.

    The trait requires only a method to be defined for the next element,
    which may be manually defined in an impl block or automatically defined
    (as in arrays and ranges).

    As a point of convenience for common situations, the for construct
    turns some collections into iterators using the .into_iter() method.
        */

    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    // Implement `Iterator` for `Fibonacci`.
    // The `Iterator` trait only requires a method to be defined for the `next` element.
    impl Iterator for Fibonacci {
        // We can refer to this type using Self::Item
        type Item = u32;

        // Here, we define the sequence using `.curr` and `.next`.
        // The return type is `Option<T>`:
        //     * When the `Iterator` is finished, `None` is returned.
        //     * Otherwise, the next value is wrapped in `Some` and returned.
        // We use Self::Item in the return type, so we can change
        // the type without having to update the function signatures.
        fn next(&mut self) -> Option<u32> {
            let new_next = self.curr + self.next;

            self.curr = self.next;
            self.next = new_next;

            // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
            // will never return `None`, and `Some` is always returned.
            Some(self.curr)
        }
    }

    // Returns a Fibonacci sequence generator
    fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }

    pub fn main() {
        // `0..3` is an `Iterator` that generates: 0, 1, and 2.
        let mut sequence = 0..3;

        println!("Four consecutive `next` calls on 0..3");
        println!("> {:?}", sequence.next());
        println!("> {:?}", sequence.next());
        println!("> {:?}", sequence.next());
        println!("> {:?}", sequence.next());

        // `for` works through an `Iterator` until it returns `None`.
        // Each `Some` value is unwrapped and bound to a variable (here, `i`).
        println!("Iterate through 0..3 using `for`");
        for i in 0..3 {
            println!("> {}", i);
        }

        // The `take(n)` method reduces an `Iterator` to its first `n` terms.
        println!("The first four terms of the Fibonacci sequence are: ");
        for i in fibonacci().take(4) {
            println!("> {}", i);
        }

        // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
        println!("The next four terms of the Fibonacci sequence are: ");
        for i in fibonacci().skip(4).take(4) {
            println!("> {}", i);
        }

        let array = [1u32, 3, 3, 7];

        // The `iter` method produces an `Iterator` over an array/slice.
        println!("Iterate the following array {:?}", &array);
        for i in array.iter() {
            println!("> {}", i);
        }
    }
}

mod drop {
    struct Droppable {
        name: &'static str,
    }

    // This trivial implementation of `drop` adds a print to console.
    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("> Dropping {}", self.name);
        }
    }

    pub fn main() {
        let _a = Droppable { name: "a" };

        // block A
        {
            let _b = Droppable { name: "b" };

            // block B
            {
                let _c = Droppable { name: "c" };
                let _d = Droppable { name: "d" };

                println!("Exiting block B");
            }
            println!("Just exited block B");

            println!("Exiting block A");
        }
        println!("Just exited block A");

        // Variable can be manually dropped using the `drop` function
        drop(_a);
        // TODO ^ Try commenting this line

        println!("end of the main function");

        // `_a` *won't* be `drop`ed again here, because it already has been
        // (manually) `drop`ed
    }
}

mod operator_overloading {
    /*
        In Rust, many of the operators can be overloaded via traits.
        That is, some operators can be used to accomplish different tasks
        based on their input arguments. This is possible because operators
        are syntactic sugar for method calls. For example, the + operator
        in a + b calls the add method (as in a.add(b)). This add method is
        part of the Add trait. Hence, the + operator can be used by any implementor
        of the Add trait.

    A list of the traits, such as Add, that overload operators can be found in core::ops.
        */

    use std::ops;

    struct Foo;
    struct Bar;

    #[derive(Debug)]
    struct FooBar;

    #[derive(Debug)]
    struct BarFoo;

    // The `std::ops::Add` trait is used to specify the functionality of `+`.
    // Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
    // The following block implements the operation: Foo + Bar = FooBar
    impl ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> FooBar {
            println!("> Foo.add(Bar) was called");

            FooBar
        }
    }

    // By reversing the types, we end up implementing non-commutative addition.
    // Here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`.
    // This block implements the operation: Bar + Foo = BarFoo
    impl ops::Add<Foo> for Bar {
        type Output = BarFoo;

        fn add(self, _rhs: Foo) -> BarFoo {
            println!("> Bar.add(Foo) was called");

            BarFoo
        }
    }

    #[derive(Debug)]
    struct Centimeters(f64);

    impl ops::Add<Centimeters> for Centimeters {
        type Output = Self;

        fn add(self, _rhs: Self) -> Centimeters {
            Centimeters(self.0 + _rhs.0)
        }
    }

    pub fn main() {
        println!("Foo + Bar = {:?}", Foo + Bar);
        println!("Bar + Foo = {:?}", Bar + Foo);
        println!(
            "Centimeters + Centimeters - Centimeters = {:?}",
            Centimeters(23.0) + Centimeters(34.0) + Centimeters(75.0)
        );
    }
}

mod returning_traits_with_dyn {
    // Returning Traits with dyn
    /*
        The Rust compiler needs to know how much space every function's return type requires.
        This means all your functions have to return a concrete type. Unlike other languages,
        if you have a trait like Animal, you can't write a function that returns Animal,
        because its different implementations will need different amounts of memory.

    However, there's an easy workaround. Instead of returning a trait object directly,
    our functions return a Box which contains some Animal. A box is just a reference to some
    memory in the heap. Because a reference has a statically-known size, and the compiler
    can guarantee it points to a heap-allocated Animal, we can return a trait from our function!

    Rust tries to be as explicit as possible whenever it allocates memory on the heap.
    So if your function returns a pointer-to-trait-on-heap in this way, you need to
    write the return type with the dyn keyword, e.g. Box<dyn Animal>.
        */

    struct Sheep {}
    struct Cow {}

    trait Animal {
        // Instance method signature
        fn noise(&self) -> &'static str;
    }

    impl Animal for Sheep {
        fn noise(&self) -> &'static str {
            "Baaaaah"
        }
    }

    impl Animal for Cow {
        fn noise(&self) -> &'static str {
            "Meeeeee"
        }
    }

    fn make_animal_shout(animal: Box<dyn Animal>) -> &'static str {
        animal.noise()
    }

    fn random_animal(random_num: f64) -> Box<dyn Animal> {
        if random_num > 2.9 {
            Box::new(Cow {})
        } else {
            Box::new(Sheep {})
        }
    }

    pub fn main_dyn() {
        let random_number = 0.234;
        let animal = random_animal(random_number);
        println!(
            "You've randomly chosen an animal, and it says {}",
            animal.noise()
        );
    }
}

mod derive {
    // `Centimeters`, a tuple struct that can be compared
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);

    // `Inches`, a tuple struct that can be printed
    #[derive(Debug)]
    struct Inches(i32);

    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;

            Centimeters(inches as f64 * 2.54)
        }
    }

    // `Seconds`, a tuple struct with no additional attributes
    #[derive(Debug, PartialEq)]
    struct Seconds(i32);

    pub fn main() {
        let _one_second = Seconds(1);

        // Error: `Seconds` can't be printed; it doesn't implement the `Debug` trait
        println!("One second looks like: {:?}", _one_second);
        // TODO ^ Try uncommenting this line

        // Error: `Seconds` can't be compared; it doesn't implement the `PartialEq` trait
        let _this_is_true = Seconds(2) == Seconds(2);
        println!("this is true, {}", _this_is_true);

        // TODO ^ Try uncommenting this line

        let foot = Inches(12);

        println!("One foot equals {:?}", foot);

        let meter = Centimeters(100.0);

        let cmp = if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

        println!("One foot is {} than one meter.", cmp);

        let cent1 = Centimeters(32.2);
        let cent2 = Centimeters(32.1);
        println!("cent2 > cent1 = {}", cent2 > cent1)
    }
}
