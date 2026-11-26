use std::io::{self, Read};
use std::net::TcpListener;

fn main() -> io::Result<()> {
    let address = std::env::var("TCP_LISTEN_ADDR").unwrap_or_else(|_| "127.0.0.1:7878".to_owned());
    let listener = TcpListener::bind(&address)?;
    println!("TCP 수신 대기 중: {}", listener.local_addr()?);

    let (mut stream, peer) = listener.accept()?;
    let mut message = String::new();
    stream.read_to_string(&mut message)?;
    println!("{peer}에서 받은 메시지: {message}");
    Ok(())
}
