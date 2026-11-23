fn print_name(name: &str) {
    println!("빌려서 읽기: {name}");
}

fn take_name(name: String) {
    println!("소유권을 넘겨받음: {name}");
}

fn main() {
    let name = String::from("온도 센서");

    // 참조(&str)로 빌려 쓰면 name의 소유권은 그대로 유지됩니다.
    print_name(&name);
    println!("빌려 쓴 뒤에도 사용 가능: {name}");

    // String을 값으로 전달하면 소유권이 이동합니다.
    take_name(name);
    // 이 지점부터 name을 사용하면 컴파일 오류가 발생합니다.
}
