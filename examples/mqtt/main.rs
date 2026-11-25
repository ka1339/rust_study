use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

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
    body.extend_from_slice(&[4, 2, 0, 30]);
    append_mqtt_string(&mut body, "rust-study-example")?;
    packet_with_header(0x10, &body)
}

fn publish_packet(topic: &str, message: &str) -> io::Result<Vec<u8>> {
    if topic.is_empty() || topic.contains(['+', '#', '\0']) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "MQTT 토픽은 비어 있거나 와일드카드를 포함할 수 없습니다",
        ));
    }

    let mut body = Vec::new();
    append_mqtt_string(&mut body, topic)?;
    body.extend_from_slice(message.as_bytes());
    packet_with_header(0x30, &body)
}

fn main() -> io::Result<()> {
    let broker = std::env::var("MQTT_BROKER").unwrap_or_else(|_| "127.0.0.1:1883".to_owned());
    let topic = std::env::var("MQTT_TOPIC").unwrap_or_else(|_| "rust-study/demo".to_owned());
    let message = "Rust에서 보낸 MQTT 메시지";
    let publish = publish_packet(&topic, message)?;

    let mut stream = TcpStream::connect(&broker)?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;
    stream.write_all(&connect_packet()?)?;

    let mut connack_header = [0; 2];
    stream.read_exact(&mut connack_header)?;
    if connack_header != [0x20, 0x02] {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("올바르지 않은 MQTT CONNACK 패킷: {connack_header:?}"),
        ));
    }

    let mut connack_body = [0; 2];
    stream.read_exact(&mut connack_body)?;
    if connack_body[0] != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Clean Session 연결에 예상하지 않은 MQTT 세션이 반환되었습니다",
        ));
    }
    if connack_body[1] != 0 {
        return Err(io::Error::new(
            io::ErrorKind::ConnectionRefused,
            format!(
                "MQTT 브로커가 연결을 거부했습니다: 코드 {}",
                connack_body[1]
            ),
        ));
    }

    stream.write_all(&publish)?;
    stream.write_all(&[0xE0, 0x00])?;
    println!("브로커 {broker}의 토픽 '{topic}'에 메시지를 발행했습니다.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{encode_remaining_length, publish_packet};

    #[test]
    fn encodes_remaining_length_boundaries() {
        assert_eq!(encode_remaining_length(127).unwrap(), [127]);
        assert_eq!(encode_remaining_length(128).unwrap(), [128, 1]);
    }

    #[test]
    fn rejects_wildcards_in_publish_topic() {
        assert!(publish_packet("devices/+", "message").is_err());
    }
}
