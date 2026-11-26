use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

type Subscriptions = Arc<Mutex<HashMap<String, Vec<mpsc::Sender<Vec<u8>>>>>>;

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
            if remaining_length > 1_048_576 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "MQTT 패킷이 예제 브로커의 1 MiB 제한을 초과했습니다",
                ));
            }
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

fn packet_with_header(header: u8, body: &[u8]) -> io::Result<Vec<u8>> {
    let mut packet = vec![header];
    packet.extend(encode_remaining_length(body.len())?);
    packet.extend_from_slice(body);
    Ok(packet)
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

fn publish_packet(topic: &str, payload: &[u8]) -> io::Result<Vec<u8>> {
    let length = u16::try_from(topic.len()).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "MQTT 토픽은 65,535바이트 이하여야 합니다",
        )
    })?;
    let mut body = length.to_be_bytes().to_vec();
    body.extend_from_slice(topic.as_bytes());
    body.extend_from_slice(payload);
    packet_with_header(0x30, &body)
}

fn handle_connect(body: &[u8]) -> io::Result<Vec<u8>> {
    let mut offset = 0;
    let protocol = mqtt_string(body, &mut offset)?;
    if protocol != "MQTT"
        || body.get(offset) != Some(&4)
        || body.get(offset + 1).is_none()
        || body[offset + 1] & 1 != 0
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "MQTT 3.1.1 CONNECT 패킷이 아닙니다",
        ));
    }
    Ok(vec![0x20, 0x02, 0, 0])
}

fn handle_subscribe(
    body: &[u8],
    subscriptions: &Subscriptions,
    publisher: &mpsc::Sender<Vec<u8>>,
) -> io::Result<Vec<u8>> {
    if body.len() < 5 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "잘린 MQTT SUBSCRIBE 패킷입니다",
        ));
    }
    let packet_id = [body[0], body[1]];
    let mut offset = 2;
    let mut granted_qos = Vec::new();
    while offset < body.len() {
        let topic = mqtt_string(body, &mut offset)?;
        if offset == body.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "SUBSCRIBE에 요청 QoS가 없습니다",
            ));
        }
        let requested_qos = body[offset];
        offset += 1;
        let is_supported =
            !topic.is_empty() && !topic.contains(['+', '#', '\0']) && requested_qos <= 2;
        if is_supported {
            subscriptions
                .lock()
                .map_err(|_| io::Error::other("MQTT 구독 잠금이 손상되었습니다"))?
                .entry(topic)
                .or_default()
                .push(publisher.clone());
            granted_qos.push(0);
        } else {
            granted_qos.push(0x80);
        }
    }

    let mut response_body = packet_id.to_vec();
    response_body.extend(granted_qos);
    packet_with_header(0x90, &response_body)
}

fn handle_publish(header: u8, body: &[u8], subscriptions: &Subscriptions) -> io::Result<()> {
    if header & 0x06 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "예제 브로커는 QoS 0 PUBLISH만 지원합니다",
        ));
    }
    let mut offset = 0;
    let topic = mqtt_string(body, &mut offset)?;
    if topic.is_empty() || topic.contains(['+', '#', '\0']) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "잘못된 MQTT 발행 토픽입니다",
        ));
    }

    let packet = publish_packet(&topic, &body[offset..])?;
    let mut subscriptions = subscriptions
        .lock()
        .map_err(|_| io::Error::other("MQTT 구독 잠금이 손상되었습니다"))?;
    if let Some(subscribers) = subscriptions.get_mut(&topic) {
        subscribers.retain(|subscriber| subscriber.send(packet.clone()).is_ok());
        if subscribers.is_empty() {
            subscriptions.remove(&topic);
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream, subscriptions: Subscriptions) -> io::Result<()> {
    let peer = stream.peer_addr()?;
    let writer = stream.try_clone()?;
    let (publisher, receiver) = mpsc::channel::<Vec<u8>>();
    thread::spawn(move || {
        let mut writer = writer;
        while let Ok(packet) = receiver.recv() {
            if let Err(error) = writer.write_all(&packet) {
                eprintln!("MQTT 클라이언트로 전달 실패: {error}");
                break;
            }
        }
    });

    let mut connected = false;
    loop {
        let (header, body) = match read_packet(&mut stream) {
            Ok(packet) => packet,
            Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => return Ok(()),
            Err(error) => return Err(error),
        };
        match header >> 4 {
            1 if !connected => {
                let response = handle_connect(&body)?;
                publisher.send(response).map_err(|_| {
                    io::Error::new(io::ErrorKind::BrokenPipe, "MQTT 응답 채널이 닫혔습니다")
                })?;
                connected = true;
                println!("{peer} MQTT 연결");
            }
            8 if connected => {
                let response = handle_subscribe(&body, &subscriptions, &publisher)?;
                publisher.send(response).map_err(|_| {
                    io::Error::new(io::ErrorKind::BrokenPipe, "MQTT 응답 채널이 닫혔습니다")
                })?;
            }
            3 if connected => handle_publish(header, &body, &subscriptions)?,
            12 if connected && body.is_empty() => {
                publisher.send(vec![0xD0, 0]).map_err(|_| {
                    io::Error::new(io::ErrorKind::BrokenPipe, "MQTT 응답 채널이 닫혔습니다")
                })?;
            }
            14 if connected && body.is_empty() => return Ok(()),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("지원하지 않거나 순서가 잘못된 MQTT 패킷: {header:#x?}"),
                ));
            }
        }
    }
}

fn main() -> io::Result<()> {
    let address = std::env::var("MQTT_BROKER_ADDR").unwrap_or_else(|_| "127.0.0.1:1883".to_owned());
    let listener = TcpListener::bind(&address)?;
    let subscriptions: Subscriptions = Arc::new(Mutex::new(HashMap::new()));
    println!(
        "간단한 MQTT 3.1.1 브로커 대기 중: {}",
        listener.local_addr()?
    );

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                let subscriptions = Arc::clone(&subscriptions);
                thread::spawn(move || {
                    if let Err(error) = handle_client(stream, subscriptions) {
                        eprintln!("MQTT 클라이언트 처리 실패: {error}");
                    }
                });
            }
            Err(error) => eprintln!("MQTT 연결 수락 실패: {error}"),
        }
    }
    Ok(())
}
