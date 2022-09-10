use std::{collections::HashMap, sync::Arc};

use mini_redis::{Command, Connection, Frame};
use tokio::{
    net::{TcpListener, TcpSocket, TcpStream},
    task,
};

type Db = Arc<::parking_lot::Mutex<HashMap<String, Vec<u8>>>>;
// type db = Arc<::parking_lot::Mutex<HashMap<String, bytes::Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let mut db = Arc::new(::parking_lot::Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();

        tokio::spawn(async move {
            process_stream(socket, db).await;
        });
    }
}

async fn process_stream(stream: TcpStream, db: Db) {
    let mut sock = Connection::new(stream);
    while let Some(frame) = sock.read_frame().await.unwrap() {
        let con = match Command::from_frame(frame).unwrap() {
            Command::Get(v) => {
                let mut db = db.lock();
                let res = db.get(v.key());
                match res {
                    Some(r) => Frame::Bulk(r.clone().into()),
                    None => Frame::Null,
                }
            }
            Command::Publish(_) => todo!(),
            Command::Set(v) => {
                let mut db = db.lock();
                db.insert(v.key().to_string(), v.value().to_vec());
                Frame::Simple("OK".into())
            }
            Command::Subscribe(_) => todo!(),
            Command::Unsubscribe(_) => todo!(),
            Command::Unknown(_) => todo!(),
        };

        sock.write_frame(&con).await.unwrap();
    }
}
