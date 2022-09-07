use crossbeam::thread;

fn main() {
    single_mutable();
}

fn single_mutable() {
    let mut greeting = String::from("Hello");
    let greeting_ref = &mut greeting;

    thread::scope(|s| {
        s.spawn(move |x| {
            *greeting_ref += " World";
            println!("{greeting_ref}"); // Prints Hello world!
        });

        // line below could cause UB or data races but compiler rejects it
        // greeting += "!!!"; // ❌ cannot mutate greeting while mutable refs exist
    })
    .unwrap();

    // can mutate greeting after the thread has joined
    greeting += "!!!";
    println!("{greeting}"); // Prints hello world
}

fn shared_immutable_data() {
    let mut greeting = String::from("Hello");
    let greeting_ref = &greeting;

    // let p = ::std::sync::atomic::AtomicI64::new(3);
    thread::scope(|scoped_thread| {
        // Spawn 3 threads
        for n in 0..3 {
            scoped_thread.spawn(move |x| {
                println!("{greeting_ref}, {n}");
            });
        }
        // line below could cause UB or data races but compiler rejects it
        // greeting += " word"; // ❌ cannot mutate greeting while immutable refs exist
    })
    .unwrap();

    // can mutate greeting after every thread has joined
    greeting += " word";
    println!("{greeting}");
    // .unwrap();
}
