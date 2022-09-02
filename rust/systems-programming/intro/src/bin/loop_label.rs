fn main() {
    loop_label();
    println!("============================================");
    println!("============================================");
    loop_infinite_return();
}

fn loop_infinite_return() {
    let n = loop {
        break 123;
    };

    println!("{n}");
}

fn loop_label() {
    'outermost: for x in 0.. {
        println!("inner0: x: {x}");
        for y in 0.. {
            println!("inner1: y: {y}");
            for z in 0.. {
                println!("inner2: z: {z}");
                if x + y + z > 1000 {
                    println!("inner: 3: {}", x + y + z);
                    break 'outermost;
                }
            }
        }
    }
}
