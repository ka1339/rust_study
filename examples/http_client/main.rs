use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let address = std::env::var("HTTP_SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_owned());
    let mut stream = TcpStream::connect(&address)?;
    write!(
        stream,
        "GET /hello HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n"
    )?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("HTTP 응답:\n{response}");
    Ok(())
}
