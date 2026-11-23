#[derive(Debug)]
struct Device {
    name: String,
    samples: Vec<i32>,
}

#[derive(Debug)]
enum Status {
    Empty,
    Normal,
    Warning(i32),
}

fn classify(samples: &[i32], warning_limit: i32) -> Status {
    match samples.iter().copied().max() {
        None => Status::Empty,
        Some(maximum) if maximum > warning_limit => Status::Warning(maximum),
        Some(_) => Status::Normal,
    }
}

fn main() {
    let device = Device {
        name: String::from("실험실 센서"),
        samples: vec![21, 23, 24, 31],
    };
    let status = classify(&device.samples, 30);

    println!("장치: {}", device.name);
    println!("측정값: {:?}", device.samples);

    // enum의 각 경우를 match로 처리합니다.
    match status {
        Status::Empty => println!("상태: 측정값 없음"),
        Status::Normal => println!("상태: 정상"),
        Status::Warning(value) => println!("상태: 경고 (최댓값 {value})"),
    }
}
