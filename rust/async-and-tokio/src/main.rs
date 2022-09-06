use bytes::Bytes;
use std::collections::HashMap;
use std::ops::SubAssign;
use std::sync::{Arc, Mutex};

// use tokio::sync::Mutex insted of std::sync::Mutex if a lock
// is being used across .await calls
// As a rule of thumb, using a synchronous mutex from within asynchronous code is fine as long as contention remains low and the lock is not held across calls to .await. Additionally, consider using parking_lot::Mutex as a faster alternative to std::sync::Mutex
// type Db = Arc<std::sync::Mutex<HashMap<String, Bytes>>>;
type Db = Arc<::parking_lot::Mutex<HashMap<String, Bytes>>>;

use mini_redis::{Connection, Frame};
use tokio::{
    net::{TcpListener, TcpStream},
    task,
};

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");
    let db = Arc::new(::parking_lot::Mutex::new(HashMap::new()));
    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();

        // Clone the db handle to the hashmap.
        let db = db.clone();

        println!("Accepted");

        // A new tasj is spawned for eacg inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // Connection, provided by `mini-redis`, handles parsing frames from the socket.
    let mut connection = Connection::new(socket);

    // Use `read_frame` to recieve a command from the connection

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock();
                // The value is stored as `Vec<u8>`
                db.insert(cmd.key().to_string(), cmd.value().to_vec().into());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock();
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`. This
                    // type will be covered later in the tutorial. For now,
                    // `&Vec<u8>` is converted to `Bytes` using `into()`.
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimpplemented {cmd:?}"),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}

async fn processv1(socket: TcpStream) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {frame:?}");

        // Respond with an error
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
