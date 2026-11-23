fn print_name(name: &str) {
    println!("빌려서 읽기: {name}");
}

fn take_name(name: String) {
    println!("소유권을 넘겨받음: {name}");
}

fn add_calibration(reading: &mut i32) {
    *reading += 1;
}

fn maximum(samples: &[i32]) -> Option<i32> {
    samples.iter().copied().max()
}

fn main() {
    let name = String::from("온도 센서");

    // 참조(&str)로 빌려 쓰면 name의 소유권은 그대로 유지됩니다.
    print_name(&name);
    println!("빌려 쓴 뒤에도 사용 가능: {name}");

    // String을 값으로 전달하면 소유권이 이동합니다.
    take_name(name);
    // 이 지점부터 name을 사용하면 컴파일 오류가 발생합니다.

    // 가변 참조는 한 번에 하나만 빌릴 수 있어 변경 중의 데이터 경쟁을 막습니다.
    let mut reading = 24;
    add_calibration(&mut reading);
    println!("보정된 측정값: {reading}");

    // 슬라이스(&[T])는 Vec 전체의 소유권 없이 연속 구간을 읽을 수 있게 합니다.
    let samples = vec![21, 23, 24, 31];
    println!("슬라이스에서 찾은 최댓값: {:?}", maximum(&samples));
}
