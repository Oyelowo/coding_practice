fn main() {
    let p = xxx("er");
    let p = xxx("er".to_string());
    let p = xxx(&"er".to_string());
}

fn xxx<T: AsRef<str>>(x: T) {
    let p = x.as_ref();
}


