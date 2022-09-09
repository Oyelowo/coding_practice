use std::collections::HashMap;

use mini_redis::{Command, Connection, Frame};
use tokio::net::{TcpListener, TcpSocket, TcpStream};

#[tokio::main]
async fn main() {
    let socket = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let mut db: HashMap<String, Vec<u8>> = HashMap::new();

    loop {
        let (tcp_stream, _) = socket.accept().await.unwrap();
        process_stream(tcp_stream, &mut db).await;
    }
}

async fn process_stream(stream: TcpStream, db: &mut HashMap<String, Vec<u8>>) {
    let mut sock = Connection::new(stream);

    while let Some(frame) = sock.read_frame().await.unwrap() {
        let con = match Command::from_frame(frame).unwrap() {
            Command::Get(v) => {
                let res = db.get(v.key());
                match res {
                    Some(r) => Frame::Bulk(r.clone().into()),
                    None => Frame::Null,
                }
            }
            Command::Publish(_) => todo!(),
            Command::Set(v) => {
                db.insert(v.key().to_string(), v.value().to_vec());
                Frame::Simple("OK".into())
            }
            Command::Subscribe(_) => todo!(),
            Command::Unsubscribe(_) => todo!(),
            Command::Unknown(_) => todo!(),
        };

        sock.write_frame(&con).await.unwrap();
    }

    // stream.ready()
}
