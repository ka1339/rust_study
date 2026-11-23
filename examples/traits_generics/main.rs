use std::fmt::Display;

struct Sensor<T> {
    name: String,
    reading: T,
}

impl<T> Sensor<T> {
    fn new(name: impl Into<String>, reading: T) -> Self {
        Self {
            name: name.into(),
            reading,
        }
    }
}

trait Summary {
    fn summarize(&self) -> String;
}

// Display를 구현한 값이면 숫자 타입을 바꾸지 않고도 센서 요약을 만들 수 있습니다.
impl<T: Display> Summary for Sensor<T> {
    fn summarize(&self) -> String {
        format!("{}의 측정값은 {}입니다.", self.name, self.reading)
    }
}

// 트레이트 바운드는 이 함수에 필요한 동작만 요구합니다.
fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

fn main() {
    let temperature = Sensor::new("실내 온도", 23.5);
    let humidity = Sensor::new("습도", 48);

    print_summary(&temperature);
    print_summary(&humidity);
}
