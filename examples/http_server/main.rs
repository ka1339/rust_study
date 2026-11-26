use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn handle_request(mut stream: TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    loop {
        let mut header = String::new();
        reader.read_line(&mut header)?;
        if header == "\r\n" || header.is_empty() {
            break;
        }
    }

    let body = "안녕하세요, HTTP 서버!";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    println!("요청: {}", request_line.trim_end());
    reader.get_mut().write_all(response.as_bytes())
}

fn main() -> io::Result<()> {
    let address = std::env::var("HTTP_LISTEN_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_owned());
    let listener = TcpListener::bind(&address)?;
    println!("HTTP 서버 대기 중: {}", listener.local_addr()?);

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => handle_request(stream)?,
            Err(error) => eprintln!("HTTP 연결 수락 실패: {error}"),
        }
    }
    Ok(())
}
