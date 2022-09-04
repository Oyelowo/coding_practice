#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // Do some async work;
        "returned value"
    });

    let out = handle.await.unwrap();
    println!("GOT: {out}");
}
