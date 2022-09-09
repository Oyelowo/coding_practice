use std::{
    marker::PhantomData,
    mem::{self, size_of},
    rc::Rc,
};

fn main() {
    // primitie
    assert_eq!(1, size_of::<u8>());
    assert_eq!(4, size_of::<u32>());
    assert_eq!(4, size_of::<i32>());
    assert_eq!(8, size_of::<f64>());

    // tuples
    assert_eq!(2, size_of::<(u8, u8)>());
    assert_eq!(8, size_of::<(u8, u32)>());

    // arrays
    assert_eq!(0, size_of::<[i32; 0]>());
    assert_eq!(12, size_of::<[i32; 3]>());

    struct Point {
        x: i32,
        y: i32,
    }

    // structs
    assert_eq!(8, size_of::<Point>());

    // enums
    assert_eq!(8, size_of::<Option<i32>>());

    // get pointer width, will be
    // 4 bytes wide on 32-bit targets or
    // 8 bytes wide on 64-bit targets
    const WIDTH: usize = size_of::<&()>();
    dbg!(WIDTH);

    // pointers to sized types are 1 width
    assert_eq!(WIDTH, size_of::<&i32>());
    assert_eq!(WIDTH, size_of::<&mut i32>());
    assert_eq!(WIDTH, size_of::<Box<i32>>());
    assert_eq!(WIDTH, size_of::<fn(i32) -> i32>());

    const DOUBLE_WIDTH: usize = 2 * WIDTH;
    dbg!(DOUBLE_WIDTH);

    dbg!(size_of::<&str>());

    // unsized struct
    struct Unsized {
        unsized_field: [i32],
    }

    // pointers to unsized types are 2 widths
    assert_eq!(DOUBLE_WIDTH, size_of::<&str>()); // slice
    assert_eq!(DOUBLE_WIDTH, size_of::<&[i32]>()); // slice
    assert_eq!(DOUBLE_WIDTH, size_of::<&dyn ToString>()); // trait object
    assert_eq!(DOUBLE_WIDTH, size_of::<Box<dyn ToString>>()); // trait object
    assert_eq!(DOUBLE_WIDTH, size_of::<&Unsized>()); // user-defined unsized type

    // unsized types
    // size_of::<str>(); // compile error
    // size_of::<[i32]>(); // compile error
    // size_of::<dyn ToString>(); // compile error
    // size_of::<Unsized>(); // compile error

    // let p = Lowo {
    //     age: todo!(),
    //     other: todo!(),
    //     opt_out_of_sync_send: PhantomData,
    // };
    dbg!(size_of::<Lowo>());
    use_generic_sized();

    use_generic_unsized();
}

type NotSendNorSync = PhantomData<Rc<()>>;

struct Lowo {
    age: u8,
    other: i128,
    opt_out_of_sync_send: NotSendNorSync,
}

// Potential alternative of opting out of a trait for a type but not yet stably supported.
// Phantom data above solves same problem by wrapping an `Rc` which is neither send nor sync
// impl !Sync for Lowo {}
// impl !Send for Lowo {}

fn use_generic_sized() {
    let val = 9i32;
    let val = String::from("er");
    let mut val = "also_works";
    let val = &mut val;
    // because generic T can -> T, &T, &mut T
    generic_sized(val);
}
fn generic_sized<T>(val: T) {}
// implicity Sized as desguared as:
// fn generic_sized<T: Sized>(val: T) {}

fn use_generic_unsized() {
    let val = 9i32;
    let val = String::from("er");
    let mut val = "also_works"; // wont work wihtouth <T: ?Sized>  because str is a string slice whose size cannot be known at compile time.
    let val: [i32; 3] = [4, 5, 6];
    let val: &[u8] = &[4, 5, 6]; //Coece sized(fixed array of 3 len) to unsized type u8 array slice
                                 // let val = &mut val;
                                 // because generic T can -> T, &T, &mut T
    generic_unsized(val);
}
fn generic_unsized<T: ?Sized>(val: &T) {}
//  Below wont work because Just `T` cannot be ?Sized because the size cannot be know at compile time
// So we can only store the reference
// fn generic_unsized<T: ?Sized>(val: T) {}

// fn generic_maybe_sized<T: Sized>(val: T) {}
