use std::io::{self, Read, Write};
use std::net::TcpStream;

fn encode_remaining_length(mut length: usize) -> io::Result<Vec<u8>> {
    if length > 268_435_455 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "MQTT 패킷 크기가 허용 범위를 초과했습니다",
        ));
    }

    let mut encoded = Vec::new();
    loop {
        let mut byte = (length % 128) as u8;
        length /= 128;
        if length > 0 {
            byte |= 0x80;
        }
        encoded.push(byte);
        if length == 0 {
            return Ok(encoded);
        }
    }
}

fn read_packet(stream: &mut TcpStream) -> io::Result<(u8, Vec<u8>)> {
    let mut header = [0];
    stream.read_exact(&mut header)?;

    let mut remaining_length = 0usize;
    let mut multiplier = 1usize;
    for _ in 0..4 {
        let mut byte = [0];
        stream.read_exact(&mut byte)?;
        remaining_length += usize::from(byte[0] & 0x7f) * multiplier;
        if byte[0] & 0x80 == 0 {
            let mut body = vec![0; remaining_length];
            stream.read_exact(&mut body)?;
            return Ok((header[0], body));
        }
        multiplier *= 128;
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "잘못된 MQTT Remaining Length입니다",
    ))
}

fn append_mqtt_string(packet: &mut Vec<u8>, value: &str) -> io::Result<()> {
    let bytes = value.as_bytes();
    let length = u16::try_from(bytes.len()).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "MQTT 문자열은 65,535바이트 이하여야 합니다",
        )
    })?;
    packet.extend_from_slice(&length.to_be_bytes());
    packet.extend_from_slice(bytes);
    Ok(())
}

fn packet_with_header(header: u8, body: &[u8]) -> io::Result<Vec<u8>> {
    let mut packet = vec![header];
    packet.extend(encode_remaining_length(body.len())?);
    packet.extend_from_slice(body);
    Ok(packet)
}

fn connect_packet() -> io::Result<Vec<u8>> {
    let mut body = Vec::new();
    append_mqtt_string(&mut body, "MQTT")?;
    body.extend_from_slice(&[4, 2, 0, 60]);
    append_mqtt_string(&mut body, "rust-study-subscriber")?;
    packet_with_header(0x10, &body)
}

fn subscribe_packet(topic: &str) -> io::Result<Vec<u8>> {
    if topic.is_empty() || topic.contains(['+', '#', '\0']) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "이 예제는 와일드카드가 없는 MQTT 토픽만 지원합니다",
        ));
    }

    let mut body = vec![0, 1];
    append_mqtt_string(&mut body, topic)?;
    body.push(0);
    packet_with_header(0x82, &body)
}

fn mqtt_string(body: &[u8], offset: &mut usize) -> io::Result<String> {
    if *offset + 2 > body.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "잘린 MQTT 문자열 길이입니다",
        ));
    }
    let length = usize::from(u16::from_be_bytes([body[*offset], body[*offset + 1]]));
    *offset += 2;
    if *offset + length > body.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "잘린 MQTT 문자열입니다",
        ));
    }
    let value = std::str::from_utf8(&body[*offset..*offset + length])
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?
        .to_owned();
    *offset += length;
    Ok(value)
}

fn main() -> io::Result<()> {
    let broker = std::env::var("MQTT_BROKER").unwrap_or_else(|_| "127.0.0.1:1883".to_owned());
    let topic = std::env::var("MQTT_TOPIC").unwrap_or_else(|_| "rust-study/demo".to_owned());
    let mut stream = TcpStream::connect(&broker)?;

    stream.write_all(&connect_packet()?)?;
    let (header, body) = read_packet(&mut stream)?;
    if header != 0x20 || body != [0, 0] {
        return Err(io::Error::new(
            io::ErrorKind::ConnectionRefused,
            format!("MQTT 브로커가 연결을 수락하지 않았습니다: {header:#x?} {body:?}"),
        ));
    }

    stream.write_all(&subscribe_packet(&topic)?)?;
    let (header, body) = read_packet(&mut stream)?;
    if header != 0x90 || body != [0, 1, 0] {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("올바르지 않은 MQTT SUBACK 패킷: {header:#x?} {body:?}"),
        ));
    }
    println!("'{topic}' 구독 중입니다. 종료하려면 Ctrl+C를 누르세요.");

    loop {
        let (header, body) = read_packet(&mut stream)?;
        if header >> 4 != 3 || header & 0x06 != 0 || body.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("예상하지 않은 MQTT 패킷: {header:#x?}"),
            ));
        }
        let mut offset = 0;
        let received_topic = mqtt_string(&body, &mut offset)?;
        let message = std::str::from_utf8(&body[offset..])
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        println!("[{received_topic}] {message}");
    }
}
