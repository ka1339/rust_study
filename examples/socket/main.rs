use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;

    let server = thread::spawn(move || -> io::Result<String> {
        let (mut stream, peer) = listener.accept()?;
        let mut message = String::new();
        stream.read_to_string(&mut message)?;
        writeln!(stream, "서버가 받은 메시지: {message}")?;
        Ok(format!("{peer}에서 받은 메시지: {message}"))
    });

    let mut client = TcpStream::connect(address)?;
    client.write_all("안녕하세요, TCP 소켓!".as_bytes())?;
    client.shutdown(std::net::Shutdown::Write)?;

    let mut response = String::new();
    client.read_to_string(&mut response)?;
    println!("클라이언트 응답: {}", response.trim_end());

    let server_message = server
        .join()
        .map_err(|_| io::Error::other("소켓 서버 스레드가 종료되었습니다"))??;
    println!("서버 로그: {}", server_message.trim_end());

    Ok(())
}
