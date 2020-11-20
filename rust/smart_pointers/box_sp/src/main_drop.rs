/// Drop traits
fn main() {
    let c = CustomSmartPointer::new(String::from("my stuff"));
    let d = CustomSmartPointer {
        data: "other stuff".to_string(),
    };
    
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointers dropped before the end of main.");
}

struct CustomSmartPointer {
    data: String,
}

impl CustomSmartPointer {
    fn new(data: String) -> Self {
        Self { data }
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}
