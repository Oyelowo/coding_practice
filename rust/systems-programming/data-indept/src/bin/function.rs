fn main() {
    let mut memory: [u8; 4096] = [0; 4096];
    let mem = &mut memory;

    let add_twice = [0x80, 0x14, 0x80, 0x14, 0x00, 0xEE];
    mem[0x100..0x106].copy_from_slice(&add_twice);

    dbg!(&mem[0x100..0x106]);
}


fn call() {
    
}