fn main() {
    println!("러스트 공부");
    println!("아래 명령으로 주제별 예제를 실행하세요:");
    println!("  cargo run --example ownership");
    println!("  cargo run --example structs_enums");
    println!("  cargo run --example option_result");
    println!("  cargo run --example iterators");
    println!("  cargo run --example threads");
    println!("  cargo run --example synchronization");
    println!("  cargo run --example data_structures");
    println!("  cargo run --example thread_pool");
    println!("  cargo run --example collections");
    println!("  cargo run --example strings");
    println!("  cargo run --example filesystem");
    println!("  cargo run --example io");
    println!("  cargo run --example system_info");
    println!("  cargo run --example smart_pointers");
    println!("  cargo run --example traits_generics");
    println!("  cargo run --example socket");
    println!("  cargo run --example socket_receiver");
    println!("  cargo run --example socket_sender -- \"보낼 메시지\"");
    println!("  cargo run --example http");
    println!("  cargo run --example http_server");
    println!("  cargo run --example http_client");
    println!("  cargo run --example mqtt");
    println!("  cargo run --example mqtt_subscriber");
    println!("  cargo run --example mqtt_broker");
    println!("  실행 순서: mqtt_broker -> mqtt_subscriber -> mqtt");
}
