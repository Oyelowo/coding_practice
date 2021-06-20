fn main() {
    intro::run();
    functions::run();
    implementation::run();
    traits::run();
    bounds::run();
    multiple_bounds::run();
    where_clauses::run();
}
mod intro {
    // Non-copyable types.
    struct Empty;
    struct Null;

    // A trait generic over `T`.
    trait DoubleDrop<T> {
        // Define a method on the caller type which takes an
        // additional single parameter `T` and does nothing with it.
        fn double_drop(self, _: T);
    }

    // Implement `DoubleDrop<T>` for any generic parameter `T` and
    // caller `U`.
    impl<T, U> DoubleDrop<T> for U {
        // This method takes ownership of both passed arguments,
        // deallocating both.
        fn double_drop(self, _: T) {}
    }

    struct T;

    fn get_it<T>(arg: T) -> T {
        arg
    }

    // A concrete type `A`.
    struct A;

    // In defining the type `Single`, the first use of `A` is not preceded by `<A>`.
    // Therefore, `Single` is a concrete type, and `A` is defined as above.
    struct Single(A);
    //            ^ Here is `Single`s first use of the type `A`.

    // Here, `<T>` precedes the first use of `T`, so `SingleGen` is a generic type.
    // Because the type parameter `T` is generic, it could be anything, including
    // the concrete type `A` defined at the top.
    struct SingleGen<T>(T);

    pub fn run() {
        let empty = Empty;
        let null = Null;

        // Deallocate `empty` and `null`.
        empty.double_drop(null);
        println!("{}", 0.1 + 0.2);

        //empty;
        //null;
        // ^ TODO: Try uncommenting these lines.

        let looo = get_it::<&str>("lowo");
        let looo = get_it::<_>("lowo");
        let looo = get_it("lowo");
        let looo = get_it::<&_>("lowo");

        // `Single` is concrete and explicitly takes `A`.
        let _s = Single(A);

        // Create a variable `_char` of type `SingleGen<char>`
        // and give it the value `SingleGen('a')`.
        // Here, `SingleGen` has a type parameter explicitly specified.
        let _char: SingleGen<char> = SingleGen('a');

        // `SingleGen` can also have a type parameter implicitly specified:
        let _t = SingleGen(A); // Uses `A` defined at the top.
        let _i32 = SingleGen(6); // Uses `i32`.
        let _char = SingleGen('ä'); // Uses `char`.
    }
}

mod functions {
    /*
        The same set of rules can be applied to functions: a type T becomes
        generic when preceded by <T>.

    Using generic functions sometimes requires explicitly specifying type parameters.
    This may be the case if the function is called where the return type is generic,
    or if the compiler doesn't have enough information to infer the necessary type parameters.

    A function call with explicitly specified type parameters looks like: fun::<A, B, ...>().
        */

    struct A; // Concrete type `A`.
    struct S(A); // Concrete type `S`.
    struct SGen<T>(T); // Generic type `SGen`.

    // The following functions all take ownership of the variable passed into
    // them and immediately go out of scope, freeing the variable.

    // Define a function `reg_fn` that takes an argument `_s` of type `S`.
    // This has no `<T>` so this is not a generic function.
    fn reg_fn(_s: S) {}

    // Define a function `gen_spec_t` that takes an argument `_s` of type `SGen<T>`.
    // It has been explicitly given the type parameter `A`, but because `A` has not
    // been specified as a generic type parameter for `gen_spec_t`, it is not generic.

    fn gen_spec_t(_s: SGen<A>) {}

    // Define a function `gen_spec_i32` that takes an argument `_s` of type `SGen<i32>`.
    // It has been explicitly given the type parameter `i32`, which is a specific type.
    // Because `i32` is not a generic type, this function is also not generic.
    fn gen_spec_i32(_s: SGen<i32>) {}

    // Define a function `generic` that takes an argument `_s` of type `SGen<T>`.
    // Because `SGen<T>` is preceded by `<T>`, this function is generic over `T`.
    fn generic<T>(_s: SGen<T>) {}

    pub fn run() {
        // Using the non-generic functions
        reg_fn(S(A)); // Concrete type.
        gen_spec_t(SGen(A)); // Implicitly specified type parameter `A`.
        gen_spec_i32(SGen(6)); // Implicitly specified type parameter `i32`.

        // Explicitly specified type parameter `char` to `generic()`.
        generic::<u8>(SGen(55));
        generic::<char>(SGen('ö'));
        generic::<&str>(SGen("onoma"));

        // Implicitly specified type parameter `char` to `generic()`.
        generic(SGen('c'));
    }
}

mod implementation {
    // Similar to functions, implementations require care to remain generic.
    struct S; // Concrete type S;
    struct GenericVal<T>(T); // Generic type `GenericVal`

    // impl of GenericVal where we explicitly specify type parameters:
    impl GenericVal<i32> {}
    impl GenericVal<&str> {}
    impl GenericVal<S> {}

    // `<T>` Must precede the type to remain generic
    impl<T> GenericVal<T> {}

    struct Val {
        val: f64,
    }

    impl Val {
        fn value(&self) -> &f64 {
            &self.val
        }
    }

    struct GenVal<T> {
        gen_val: T,
    }

    impl<T> GenVal<T> {
        fn value(&self) -> &T {
            &self.gen_val
        }
    }

    pub fn run() {
        let x = Val { val: 3.0 };
        let y = GenVal { gen_val: 3i32 };

        println!("{}, {}", x.value(), y.value());
    }
}

mod traits {
    /*
    Of course traits can also be generic.
    Here we define one which reimplements the Drop trait as a
    generic method to drop itself and an input.
    */

    // Non-copyable types.
    struct Empty;
    struct Null;

    // A trait generic over `T`.
    trait DoubleDrop<T> {
        // Define a method on the caller type which takes an
        // additional single parameter `T` and does nothing with it.
        fn double_drop(self, _: T);
    }

    // Implement `DoubleDrop<T>` for any generic parameter `T` and
    // caller `U`.
    impl<T, U> DoubleDrop<T> for U {
        // This method takes ownership of both passed arguments,
        // deallocating both.
        fn double_drop(self, _: T) {}
    }

    pub fn run() {
        let empty = Empty;
        let null = Null;

        // Deallocate `empty` and `null`.
        empty.double_drop(null);

        //empty;
        //null;
        // ^ TODO: Try uncommenting these lines.
    }
}

mod bounds {
    use std::fmt::Display;

    /*
    When working with generics, the type parameters often must use
    traits as bounds to stipulate what functionality a type implements.
    For example, the following example uses the trait Display to print
    and so it requires T to be bound by Display; that is, T must implement Display.
    */

    // Define a function `printer` that takes a generic type `T` which
    // must implement trait `Display`.
    fn printer<T: Display>(t: T) {
        println!("{}", t);
    }
    fn printer2<T>(t: T)
    where
        T: Display,
    {
        println!("{}", t);
    }

    /*
    Bounding restricts the generic to types that conform to the bounds. That is:
    */
    struct S<T: Display>(T);

    // A trait which implements the print marker: `{:?}`.
    use std::fmt::Debug;

    /*
    Another effect of bounding is that generic instances are allowed to
    access the methods of traits specified in the bounds. For example:
    */
    trait HasArea {
        fn area(&self) -> f64;
    }

    #[derive(Debug)]
    struct Rectangle {
        length: f64,
        height: f64,
    }

    #[allow(dead_code)]
    struct Triangle {
        length: f64,
        height: f64,
    }

    impl HasArea for Rectangle {
        fn area(&self) -> f64 {
            self.length * self.height
        }
    }

    // The generic `T` must implement `Debug`. Regardless
    // of the type, this will work properly.
    fn print_debug<T: Debug>(t: &T) {
        println!("{:?}", t);
    }

    fn area<T: HasArea>(t: &T) -> f64 {
        t.area()
    }

    // Testcase: empty bounds
    /*
    A consequence of how bounds work is that even if a trait doesn't
    include any functionality, you can still use it as a bound. Eq and
    Copy are examples of such traits from the std library.
        */
    struct Cardinal;
    struct BlueJay;
    struct Turkey;

    trait Red {}
    trait Blue {}

    impl Red for Cardinal {}
    impl Blue for BlueJay {}

    // These functions are only valid for types which implement these
    // traits. The fact that the traits are empty is irrelevant.
    fn red<T: Red>(_: &T) -> &'static str {
        "red"
    }

    fn blue<T: Blue>(_: &T) -> &'static str {
        "blue"
    }

    pub fn run() {
        // Error! `Vec<T>` does not implement `Display`. This
        // specialization will fail.
        // let s = S(vec![1]);

        let rectangle = Rectangle {
            length: 34.5,
            height: 52.2,
        };
        let _triangle = Triangle {
            length: 24.1,
            height: 48.2,
        };

        print_debug(&rectangle);
        println!("Area: {:?}", area(&rectangle));

        //print_debug(&_triangle);
        //println!("Area: {}", area(&_triangle));
        // ^ TODO: Try uncommenting these.
        // | Error: Does not implement either `Debug` or `HasArea`.

        let cardinal = Cardinal;
        let blue_jay = BlueJay;
        let _turkey = Turkey;

        // `red()` won't work on a blue jay nor vice versa
        // because of the bounds.
        println!("A cardinal is {}", red(&cardinal));
        println!("A blue jay is {}", blue(&blue_jay));

        //println!("A turkey is {}", red(&_turkey));
        // ^ TODO: Try uncommenting this line.
    }

    /*
    As an additional note, where clauses can also be used to apply bounds in some cases to be more expressive.
    */
}

mod multiple_bounds {
    use std::fmt::{Debug, Display};

    /*
    Multiple bounds for a single type can be applied with a +.
    Like normal, different types are separated with ,.
    */
    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("Debug: `{:?}`", t);
        println!("Display: `{}`", t);
    }

    fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
        println!("t: `{:?}`", t);
        println!("u: `{:?}`", u);
    }
    pub fn run() {
        let string = "hello world";
        let array = [1, 2, 3];
        let vec = vec![1, 2, 3];

        compare_prints(&string);
        // compare_prints(&array);
        // TODO ^ Try uncommenting this.
        compare_types(&array, &vec);
    }
}

mod where_clauses {
    /*
    A bound can also be expressed using a where clause immediately
    before the opening {, rather than at the type's first mention.
    Additionally, where clauses can apply bounds to arbitrary types,
    rather than just to type parameters.

    */

    /*      Some cases that a where clause is useful:

    When specifying generic types and bounds separately is clearer: */
    // impl<A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType

    // Expressing bounds with a `where` clause
    /*   impl<A, D> for YourType<A, D> where
    A:TraitB + TraitC,
    D:TraitE + TraitF {} */

    /*
    When using a where clause is more expressive than using normal syntax.
    The impl in this example cannot be directly expressed without a where clause:
     */
    use std::fmt::Debug;
    trait PrintInOption {
        fn print_in_option(self);
    }

    // Because we would otherwise have to express this as `T: Debug` or
    // use another method of indirect approach, this requires a `where` clause:
    impl<T> PrintInOption for T
    where
        Option<T>: Debug,
        // T: Debug // wrong otherwise for Some(self)
        // We want `Option<T>: Debug` as our bound because that is what's
        // being printed. Doing otherwise would be using the wrong bound.
    {
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }
    pub fn run() {
        let vec = vec![1, 2, 3];

        vec.print_in_option();
    }
}

mod new_type {
    const DAYS_IN_A_YEAR: i64 = 365;
    /*
        The newtype idiom gives compile time guarantees that the right
        type of value is supplied to a program.

    For example, an age verification function that checks age in years,
    must be given a value of type Years.
        */

    struct Years(i64);

    impl Years {
        pub fn to_days(&self) -> Days {
            Days(self.0 * DAYS_IN_A_YEAR)
        }
    }
    struct Days(i64);

    impl Days {
        pub fn to_years(&self) -> Years {
            Years(self.0 / DAYS_IN_A_YEAR)
        }
    }

    fn old_enough(age: &Years) -> bool {
        age.0 >= 18
    }

    pub fn run() {
        let age = Years(5);
        let age_days = age.to_days();
        println!("Old enough {}", old_enough(&age));
        println!("Old enough {}", old_enough(&age_days.to_years()));
        // println!("Old enough {}", old_enough(&age_days));

        /*
                Uncomment the last print statement to observe that the type supplied must be Years.

        To obtain the newtype's value as the base type, you may use the tuple
        or destructuring syntax like so:
                */

        let years = Years(42);
        let years_as_primitive_1: i64 = years.0;
        let Years(years_as_primitive_2) = years;
        // Obtain as reference
        let Years(ref years_as_primitive_3) = years;
    }
}
