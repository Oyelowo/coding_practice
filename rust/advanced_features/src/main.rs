// Unsafe Rust

fn main() {
    // Implementing an Unsafe Trait

    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }
}

static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn _main5() {
    // Accessing or Modifying a Mutable Static Variable
    println!("name is: {}", HELLO_WORLD);
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}
fn _main4() {
    // Using extern Functions to Call External Code

    unsafe {
        println!("Absolute value  of -3 according to C:{}", abs(-3));
    }

    /*
        Calling Rust Functions from Other Languages
    We can also use extern to create an interface that allows other languages to call Rust functions. Instead of an extern block, we add the extern keyword and specify the ABI to use just before the fn keyword. We also need to add a #[no_mangle] annotation to tell the Rust compiler not to mangle the name of this function. Mangling is when a compiler changes the name we’ve given a function to a different name that contains more information for other parts of the compilation process to consume but is less human readable. Every programming language compiler mangles names slightly differently, so for a Rust function to be nameable by other languages, we must disable the Rust compiler’s name mangling.

    In the following example, we make the call_from_c function accessible from C code, after it’s compiled to a shared library and linked from C:


    This usage of extern does not require unsafe.
    */

    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
}

fn _main3() {
    // Creating a Safe Abstraction over Unsafe Code
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // (&mut slice[..mid], &mut slice[mid..])
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn _main2() {
    // Calling an Unsafe Function or Method
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

fn _main1() {
    // Dereferencing a Raw Pointer
    println!("Hello, world!");
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r1 is: {}", *r2);
    }

    /*
    Next, we’ll create a raw pointer whose validity we can’t be so certain of. Listing 19-2 shows how to create a raw pointer to an arbitrary location in memory. Trying to use arbitrary memory is undefined: there might be data at that address or there might not, the compiler might optimize the code so there is no memory access, or the program might error with a segmentation fault. Usually, there is no good reason to write code like this, but it is possible.
    */
    let address = 0x012345usize;
    let r = address as *const i32;
}
