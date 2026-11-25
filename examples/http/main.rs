use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_request(mut stream: TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    let mut headers = Vec::new();
    loop {
        let mut header = String::new();
        reader.read_line(&mut header)?;
        if header == "\r\n" || header.is_empty() {
            break;
        }
        headers.push(header.trim_end().to_owned());
    }

    println!("요청: {}", request_line.trim_end());
    for header in headers {
        println!("헤더: {header}");
    }

    let body = "안녕하세요, HTTP!";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    reader.get_mut().write_all(response.as_bytes())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;

    let server = thread::spawn(move || -> io::Result<()> {
        let (stream, _) = listener.accept()?;
        handle_request(stream)
    });

    let mut client = TcpStream::connect(address)?;
    write!(
        client,
        "GET /hello HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n"
    )?;

    let mut response = String::new();
    client.read_to_string(&mut response)?;
    println!("HTTP 응답:\n{response}");

    server
        .join()
        .map_err(|_| io::Error::other("HTTP 서버 스레드가 종료되었습니다"))??;
    Ok(())
}
