use std::rc::Rc;
use tokio::task::yield_now;

/*
Send bound
Tasks spawned by tokio::spawn must implement Send. This allows the Tokio runtime to
move the tasks between threads while they are suspended at an .await.

Tasks are Send when all data that is held across .await calls is Send.
This is a bit subtle. When .await is called, the task yields back to the scheduler.
The next time the task is executed, it resumes from the point it last yielded.
To make this work, all state that is used after .await must be saved by the task.
If this state is Send, i.e. can be moved across threads,
then the task itself can be moved across threads.
Conversely, if the state is not Send, then neither is the task.

For example, this works:
*/

#[tokio::main]
async fn main() {
    // This compiles
    tokio::spawn(async {
        // The scope forces `rc` to drop before `.await`.

        {
            let rc = Rc::new("hello");
            println!("{rc}");
        }

        // `rc` is no longer used. It is **not** persisted when
        // the task yields to the scheduler
        yield_now().await;
    });

    // but not this does not

    // tokio::spawn(async {
    //     // The scope forces `rc` to drop before `.await`.

    //     let rc = Rc::new("hello");
    //     //  The fix
    //     // let rc = std::sync::Arc::new("hello");

    //     // `rc` is no longer used. It is **not** persisted when
    //     // the task yields to the scheduler
    //     yield_now().await;
    //     println!("{rc}");
    // });
}
