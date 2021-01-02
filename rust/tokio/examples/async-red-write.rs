use tokio::fs::File;
use tokio::{
    self,
    io::{self, AsyncReadExt, AsyncWriteExt},
};

fn main() {
    read().unwrap();
    read_to_end().unwrap();
    write().unwrap();
    write_all().unwrap();
    read_copy().unwrap();


}

#[tokio::main]
async fn read() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;

    let mut buffer = [0, 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer[..]).await?;

    println!("The bytes: {:?}", &buffer[..n]);
    println!("The byteees: {:?}", buffer);
    Ok(())
}

#[tokio::main]
async fn read_to_end() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;

    let mut buffer = Vec::new();

    // read the whole file
    let n = f.read_to_end(&mut buffer).await?;

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}

#[tokio::main]
async fn write() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;

    // Writes some prefix of the byte string, but not necessarily all of it.
    let n = file.write(b"some bytes").await?;
    println!("Wrote the first {} bytes of 'some bytes'.", n);

    Ok(())
}

#[tokio::main]
async fn write_all() -> io::Result<()> {
    let mut buffer = File::create("foo.txt").await?;

    let n = buffer.write_all(b"some bytes").await?;
    println!("Wrote the first {:?} bytes of 'some bytes'.", n);

    Ok(())
}

#[tokio::main]
async fn read_copy() -> io::Result<()> {
    let mut reader: &[u8] = b"hello Oyelowo";
    let mut file = File::create("foo.txt").await?;

    let n = io::copy(&mut reader, &mut file).await?;
    println!("Wrote the first {:?} bytes of 'some bytes'.", n);

    Ok(())
}
