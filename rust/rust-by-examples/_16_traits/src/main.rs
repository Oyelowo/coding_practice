fn main() {
    derive::main();
    returning_traits_with_dyn::main_dyn();
    operator_overloading::main();
    drop::main();
    iterators::main();
    impl_trait::main();
    clone::main();
}

mod clone {
    /*
        When dealing with resources, the default behavior is to transfer them
        during assignments or function calls. However, sometimes we need to
        make a copy of the resource as well.

    The Clone trait helps us do exactly this. Most commonly, we can use the .clone()
    method defined by the Clone trait.
        */

    // A unit struct without resources
    #[derive(Debug, Clone, Copy)]
    struct Unit;

    // A tuple struct with resources that implements the `Clone` trait
    #[derive(Debug, Clone)]
    struct Pair(Box<i32>, Box<i32>);

    pub fn main() {
        // Instantiate `Unit`
        let unit = Unit;

        // Copy `Unit`, there are no resources to move
        let copied_unit = unit;

        // Both `Unit`s can be used independently
        println!("original: {:?}", unit);
        println!("copy: {:?}", copied_unit);

        // Instantiate `Pair`
        let pair = Pair(Box::new(1), Box::new(2));
        println!("original: {:?}", pair);

        // Move `pair` into `moved_pair`, moves resources
        let moved_pair = pair;
        println!("moved: {:?}", moved_pair);

        // Error! `pair` has lost its resources
        //println!("original: {:?}", pair);
        // TODO ^ Try uncommenting this line

        // Clone `moved_pair` into `cloned_pair` (resources are included)
        let cloned_pair = moved_pair.clone();
        // Drop the original pair using std::mem::drop
        drop(moved_pair);

        // Error! `moved_pair` has been dropped
        // println!("copy: {:?}", moved_pair);
        // TODO ^ Try uncommenting this line

        // The result from .clone() can still be used!
        println!("clone: {:?}", cloned_pair);
    }
}

mod impl_trait {

    /*
        If your function returns a type that implements MyTrait,
        you can write its return type as -> impl MyTrait. This can help simplify
        your type signatures quite a lot!
    */

    use std::iter;
    use std::vec::IntoIter;

    // This function combines two `Vec<i32>` and returns an iterator over it.
    // Look how complicated its return type is!
    fn combine_vecs_explicit_return_type(
        v: Vec<i32>,
        u: Vec<i32>,
    ) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    // This is the exact same function, but its return type uses `impl Trait`.
    // Look how much simpler it is!
    fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    pub fn main() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5];
        let mut v3 = combine_vecs(v1, v2);
        assert_eq!(Some(1), v3.next());
        assert_eq!(Some(2), v3.next());
        assert_eq!(Some(3), v3.next());
        assert_eq!(Some(4), v3.next());
        assert_eq!(Some(5), v3.next());
        println!("all done");

        let plus_one = make_adder_function(1);
        assert_eq!(plus_one(2), 3);
    }

    /*
    More importantly, some Rust types can't be written out. For example,
    every closure has its own unnamed concrete type. Before impl Trait syntax,
     you had to allocate on the heap in order to return a closure. But now you
     can do it all statically, like this:
    */

    // Returns a function that adds `y` to its input
    fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
        let closure = move |x: i32| x + y;
        closure
    }

    /*
    You can also use impl Trait to return an iterator that uses map or
    filter closures! This makes using map and filter easier. Because closure
    types don't have names, you can't write out an explicit return type if
    your function returns iterators with closures. But with impl Trait you
    can do this easily:
    */

    fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
        numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
    }
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
        fn next(&mut self) -> Option<Self::Item> {
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
