use tokio::task;

#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3];
    // You need to move to make the task static so that it owns
    // all the data within its scope
    task::spawn(async move {
        println!("Here 's a vec: {v:?}");
    });
}
