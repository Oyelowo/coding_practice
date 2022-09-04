async fn say_world() {
    println!("world");
}


#[tokio::main]
async fn main() {
    let op = say_world();
    let ap = say_world();

    println!("Should print first: Hello");


    op.await;
    ap.await;
}
