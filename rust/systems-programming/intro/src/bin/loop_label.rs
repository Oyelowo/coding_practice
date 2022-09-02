fn main() {
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
