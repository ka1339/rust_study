use std::io::{self, Write};
use std::net::{Shutdown, TcpStream};

fn main() -> io::Result<()> {
    let address =
        std::env::var("TCP_RECEIVER_ADDR").unwrap_or_else(|_| "127.0.0.1:7878".to_owned());
    let message = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "안녕하세요, TCP 수신자!".to_owned());

    let mut stream = TcpStream::connect(&address)?;
    stream.write_all(message.as_bytes())?;
    stream.shutdown(Shutdown::Write)?;
    println!("{address}로 메시지를 보냈습니다: {message}");
    Ok(())
}
