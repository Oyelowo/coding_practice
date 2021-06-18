fn main() {
    methods_1();
    closures();
    closure_capturing();
    closure_move();
    closure_as_input_parameters();
    type_anonymity();
    input_functions();
    closure_as_output_parameters();
    iterator_any();
    higher_order_function();
    diverging_function();
    diverging_function2();
}

////////////////////////////////////
// Closures
/*
Closures are functions that can capture the enclosing environment.
For example, a closure that captures the x variable:


|val| val + x
The syntax and capabilities of closures make them very convenient
for on the fly usage. Calling a closure is exactly like calling a function.
However, both input and return types can be inferred and input variable names must be specified.

Other characteristics of closures include:

- using || instead of () around input variables.
- optional body delimination ({}) for a single expression (mandatory otherwise).
- the ability to capture the outer environment variables.

*/

fn closures() {
    // Increment via closures and functions.
    fn function(i: i32) -> i32 {
        i + 1
    }

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
}

///////////////////////////////////////////////////////////////////
// Capturing (Closure)
/*
Capturing
Closures are inherently flexible and will do what the functionality requires
to make the closure work without annotation. This allows capturing to flexibly
adapt to the use case, sometimes moving and sometimes borrowing. Closures can capture variables:

- by reference: &T
- by mutable reference: &mut T
- by value: T
They preferentially capture variables by reference and only go lower when required.
*/
fn closure_capturing() {
    use std::mem;

    let color = String::from("green");

    // A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time.
    //
    // `println!` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`.
    let _reborrow = &color;

    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;

    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow.
    inc();

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.

    // let _reborrow = &count;
    // ^ TODO: try uncommenting this line.
    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;

    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    // consume();
    // ^ TODO: Try uncommenting this line.
}

// Using move before vertical pipes forces closure to take ownership of captured variables:
fn closure_move() {
    // `Vec` has non-copy semantics.
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());
    // ^ Uncommenting above line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after it
    // has been moved.

    // Removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting above line will not cause an error.
}

////////////////////////////////////////////////////////////////
// As input parameters
/*
While Rust chooses how to capture variables on the fly mostly without type annotation,
this ambiguity is not allowed when writing functions. When taking a closure as an input parameter,
the closure's complete type must be annotated using one of a few traits. In order of decreasing restriction, they are:

Fn: the closure captures by reference (&T)
FnMut: the closure captures by mutable reference (&mut T)
FnOnce: the closure captures by value (T)
On a variable-by-variable basis, the compiler will capture variables in the least restrictive manner possible.

For instance, consider a parameter annotated as FnOnce. This specifies that the closure may capture by

&T, &mut T, or T, but the compiler will ultimately choose based on how the captured variables are used in the closure.

This is because if a move is possible, then any type of borrow should also be possible.
Note that the reverse is not true. If the parameter is annotated as Fn, then capturing variables by &mut T or T are not allowed.

In the following example, try swapping the usage of Fn, FnMut, and FnOnce to see what happens:
*/

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F)
where
    // The closure takes no input and returns nothing. Same as FnOnce() -> ()
    F: FnOnce(), // ^ TODO: Try changing this to `Fn` or `FnMut`.
{
    f()
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn closure_as_input_parameters() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

/////////////////////////////////
// Type anonymity
/*
Closures succinctly capture variables from enclosing scopes.
Does this have any consequences? It surely does. Observe how using a closure as
a function parameter requires generics, which is necessary because of how they are defined:

*/

// `F` must be generic.
fn apply2<F>(f: F)
where
    F: FnOnce(),
{
    f();
}
/*
When a closure is defined, the compiler implicitly creates a new anonymous
structure to store the captured variables inside, meanwhile implementing the
functionality via one of the traits: Fn, FnMut, or FnOnce for this unknown type.
This type is assigned to the variable which is stored until calling.

Since this new type is of unknown type, any usage in a function will require generics.
However, an unbounded type parameter <T> would still be ambiguous and not be allowed.
Thus, bounding by one of the traits: Fn, FnMut, or FnOnce (which it implements) is sufficient to specify its type.

*/

// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`.
fn apply3<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn type_anonymity() {
    let x = 7;

    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    let print = || println!("{}", x);

    apply(print);
}

// Input functions
/*
Since closures may be used as arguments, you might wonder if the same can be said about functions.
And indeed they can! If you declare a function that takes a closure as parameter,
then any function that satisfies the trait bound of that closure can be passed as a parameter.
*/
// Define a function which takes a generic `F` argument
// bounded by `Fn`, and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("I'm a function!");
}

fn input_functions() {
    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
/*
As an additional note, the Fn, FnMut, and FnOnce traits dictate how a closure captures variables from the enclosing scope.
*/

/////////////////////////////////
// As output parameters
/*
Closures as input parameters are possible, so returning closures as output parameters
should also be possible. However, anonymous closure types are, by definition,
unknown, so we have to use impl Trait to return them.

The valid traits for returning a closure are:

Fn
FnMut
FnOnce
Beyond this, the move keyword must be used, which signals that all captures occur by value.
This is required because any captures by reference would be dropped as soon as the function exited,
leaving invalid references in the closure.
*/
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn closure_as_output_parameters() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}

////////////////////////////////
// Closures Examples std
// Iterator::any
/*
Iterator::any is a function which when passed an iterator,
will return true if any element satisfies the predicate. Otherwise false. Its signature:
*/
/* pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `any` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn any<F>(&mut self, f: F) -> bool
    where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `Self::Item` states it takes
        // arguments to the closure by value.
        F: FnMut(Self::Item) -> bool,
    {

    }
} */

fn iterator_any() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));

    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`.
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // `into_iter()` for arrays unusually yields `&i32`.
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}

/////////////////////////////////////
// Searching through iterators
/*
Iterator::find is a function which iterates over an iterator and
searches for the first value which satisfies some condition.
If none of the values satisfy the condition, it returns None. Its signature:
*/

/* pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `find` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `&Self::Item` states it takes
        // arguments to the closure by reference.
        P: FnMut(&Self::Item) -> bool {}
} */

fn iterator_find() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`.
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`.
    let mut into_iter = vec2.into_iter();

    // `iter()` for vecs yields `&i32`, and we want to reference one of its
    // items, so we have to destructure `&&i32` to `i32`
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    // `into_iter()` for vecs yields `i32`, and we want to reference one of
    // its items, so we have to destructure `&i32` to `i32`
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    // `into_iter()` for arrays unusually yields `&i32`
    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&&x| x == 2)
    );

    let vec = vec![1, 9, 3, 3, 13, 2];

    let index_of_first_even_number = vec.iter().position(|x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));

    let index_of_first_negative_number = vec.iter().position(|x| x < &0);
    assert_eq!(index_of_first_negative_number, None);
}

////////////////////////////////////////////////////////////////
// Higher Order Functions
/*
Rust provides Higher Order Functions (HOF). These are functions that take one or more
functions and/or produce a more useful function. HOFs and lazy iterators give Rust its functional flavor.
*/
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn higher_order_function() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;
    // Iterate: 0, 1, 2, ... to infinity
    for n in 0.. {
        // Square the number
        let n_squared = n * n;

        if n_squared >= upper {
            // Break loop if exceeded the upper limit
            break;
        } else if is_odd(n_squared) {
            // Accumulate value, if it's odd
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    let sum_of_squares: u32 = (0..)
        .map(|a| a * a)
        .take_while(|&x| x <= upper)
        .filter(|x| is_odd(*x))
        .fold(0, |acc, val| acc + val);

    println!("My solutionhahahah, {}", sum_of_squares);

    // Functional approach
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // All natural numbers squared
        .take_while(|&n_squared| n_squared < upper) // Below upper limit
        .filter(|&n_squared| is_odd(n_squared)) // That are odd
        .fold(0, |acc, n_squared| acc + n_squared); // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

////////////////////////////////////////////////////////////////
// Methods
/*
Methods are functions attached to objects.
These methods have access to the data of the object and its other
methods via the self keyword. Methods are defined under an impl block.
*/
struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` methods go in here
impl Point {
    // This is a static method
    // Static methods don't need to be called by an instance
    // These methods are generally used as constructors
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another static method, taking two arguments:
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is an instance method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    fn destroy(self) {
        // Destructure `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
    }
}

fn methods_1() {
    let rectangle = Rectangle {
        // Static methods are called using double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Instance methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line

    // Okay! Mutable objects can call mutable methods
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    //pair.destroy();
    // TODO ^ Try uncommenting this line
}

// Functions that "don't" return a value, actually return the unit type `()`

/*
// When a function returns `()`, the return type can be omitted from the
// signature
*/

////////////////////////////////
// Diverging functions
fn diverging_function() -> ! {
    panic!("This call never returns.");
}

/*
Although this might seem like an abstract concept, it is in fact very useful and often handy.
The main advantage of this type is that it can be cast to any other one and therefore used at places
where an exact type is required, for instance in match branches. This allows us to write code like this:
*/
fn diverging_function2() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of the "addition" variable.
            let addition: u32 = match i % 2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!(
        "Sum of odd numbers up to 9 (excluding): {}",
        sum_odd_numbers(9)
    );
}

/*
It is also the return type of functions that loop forever (e.g. loop {})
like network servers or functions that terminate the process (e.g. exit()).
*/
