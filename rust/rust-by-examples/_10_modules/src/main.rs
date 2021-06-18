// Modules
/*
Rust provides a powerful module system that can be used to hierarchically
split code in logical units (modules), and manage visibility (public/private) between them.

A module is a collection of items: functions, structs, traits, impl blocks, and even other modules.
*/

// File hierarchy

// This declaration will look for a file named `my.rs` or `my/mod.rs` and will
// insert its contents inside a module named `my` under this scope
mod my_module;

fn function_hierarchy() {
    println!("called `function()`");
}

fn file_hierarchy() {
    my_module::function();

    function_hierarchy();

    my_module::indirect_access();

    my_module::nested::function();
}

fn main() {
    /*     println!("Hello, world!");
    module_visibility();
    struct_visibility();
    use_keyword();
    self_and_super(); */
    file_hierarchy();
}

// Visibility
/*
By default, the items in a module have private visibility,
but this can be overridden with the pub modifier. Only the public
items of a module can be accessed from outside the module scope.
*/

// A module named `my_mod`
mod my_mod {
    // Items in modules default to private visibility.
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // Use the `pub` modifier to override default visibility.
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // Items can access other items in the same module,
    // even when private.
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // Modules can also be nested
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // Functions declared using `pub(in path)` syntax are only visible
        // within the given path. `path` must be a parent or ancestor module
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // Functions declared using `pub(self)` syntax are only visible within
        // the current module, which is the same as leaving them private
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // Functions declared using `pub(super)` syntax are only visible within
        // the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // Private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn module_visibility() {
    // Modules allow disambiguation between items that have the same name.
    function();
    my_mod::function();

    // Public items, including those inside nested modules, can be
    // accessed from outside the parent module.
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();

    // pub(in path) items can only be called from within the module specified
    // Error! function `public_function_in_my_mod` is private
    //my_mod::nested::public_function_in_my_mod();
    // TODO ^ Try uncommenting this line

    // Private items of a module cannot be directly accessed, even if
    // nested in a public module:

    // Error! `private_function` is private
    //my_mod::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_function` is private
    //my_mod::nested::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::restricted_function();
    // TODO ^ Try uncommenting this line
}

// Struct visibility
/*
Structs have an extra level of visibility with their fields.
The visibility defaults to private, and can be overridden with the pub modifier.
This visibility only matters when a struct is accessed from outside the module
where it is defined, and has the goal of hiding information (encapsulation).
*/

mod my {
    // A public struct with a public field of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
    }
}

fn struct_visibility() {
    // Public structs with public fields can be constructed as usual
    let open_box = my::OpenBox {
        contents: "public information",
    };

    // and their fields can be normally accessed.
    println!("The open box contains: {}", open_box.contents);

    // Public structs with private fields cannot be constructed using field names.
    // Error! `ClosedBox` has private fields
    //let closed_box = my::ClosedBox { contents: "classified information" };
    // TODO ^ Try uncommenting this line

    // However, structs with private fields can be created using
    // public constructors
    let _closed_box = my::ClosedBox::new("classified information");

    // and the private fields of a public struct cannot be accessed.
    // Error! The `contents` field is private
    //println!("The closed box contains: {}", _closed_box.contents);
    // TODO ^ Try uncommenting this line
}

///////////////////////////////////////////////////////////////////
// The use declaration
/*
The use declaration can be used to bind a full path to a new name, for easier access. It is often used like this:
*/

// Bind the `deeply::nested::function` path to `other_function`.
use deeply::nested::function_test as other_function;

fn function_test() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function_test() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn use_keyword() {
    // Easier access to `deeply::nested::function`
    other_function();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::deeply::nested::function_test;

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        function();

        println!("Leaving block");
    }

    function_test();
}

// ----------------------------------------------------------------
/////////////////////////////////////
// super and self
/*
The super and self keywords can be used in the path to remove ambiguity when
accessing items and to prevent unnecessary hardcoding of paths.
*/

fn function_() {
    println!("called `function_()`");
}

mod cool {
    pub fn function_() {
        println!("called `cool::function_()`");
    }
}

mod mod_self_super {
    fn function_() {
        println!("called `my::function_()`");
    }

    mod cool {
        pub fn function_() {
            println!("called `my::cool::function_()`");
        }
    }

    pub fn indirect_call() {
        // Let's access all the function_s named `function_` from this scope!
        print!("called `my::indirect_call()`, that\n> ");

        // The `self` keyword refers to the current module scope - in this case `my`.
        // Calling `self::function_()` and calling `function_()` directly both give
        // the same result, because they refer to the same function_.
        self::function_();
        function_();

        // We can also use `self` to access another module inside `my`:
        self::cool::function_();

        // The `super` keyword refers to the parent scope (outside the `my` module).
        super::function_();

        // This will bind to the `cool::function_` in the *crate* scope.
        // In this case the crate scope is the outermost scope.
        {
            use crate::cool::function_ as root_function_;
            root_function_();
        }
    }
}

fn self_and_super() {
    mod_self_super::indirect_call();
}
