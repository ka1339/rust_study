use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // env를 통해 현재 디렉터리, 프로세스 인자, 환경 변수를 읽을 수 있습니다.
    println!("현재 디렉터리: {}", std::env::current_dir()?.display());
    let arguments: Vec<_> = std::env::args().skip(1).collect();
    println!("전달된 인자: {arguments:?}");
    println!(
        "RUST_LOG: {}",
        std::env::var("RUST_LOG").unwrap_or_else(|_| String::from("(설정되지 않음)"))
    );

    // Instant는 경과 시간을 재고, SystemTime은 벽시계 시간을 다룰 때 사용합니다.
    let start = Instant::now();
    let total: u64 = (1..=100_000).sum();
    println!("계산 결과: {total}, 경과 시간: {:?}", start.elapsed());

    let unix_seconds = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    println!("현재 Unix 시간: {unix_seconds}초");
    println!("250밀리초: {:?}", Duration::from_millis(250));

    Ok(())
}
