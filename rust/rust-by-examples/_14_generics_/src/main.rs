use std::fmt::Display;

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

fn main() {
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

    fn run() {
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

mod Implementation {
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

    fn run() {
        let x = Val { val: 3.0 };
        let y = GenVal { gen_val: 3i32 };

        println!("{}, {}", x.value(), y.value());
    }
}
