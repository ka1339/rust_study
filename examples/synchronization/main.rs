use std::sync::{Arc, Barrier, Condvar, Mutex, OnceLock, RwLock};
use std::thread;

fn main() {
    // Mutex는 한 번에 하나의 쓰레드만 공유 값을 변경하도록 보호합니다.
    let counter = Arc::new(Mutex::new(0));
    let mut workers = Vec::new();

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        workers.push(thread::spawn(move || {
            let mut value = counter
                .lock()
                .expect("Mutex가 잠금 중에 오염되면 안 됩니다.");
            *value += 1;
        }));
    }

    for worker in workers {
        worker
            .join()
            .expect("작업 쓰레드가 정상 종료되어야 합니다.");
    }
    println!(
        "Mutex 카운터: {}",
        *counter
            .lock()
            .expect("Mutex가 잠금 중에 오염되면 안 됩니다.")
    );

    // RwLock은 여러 읽기와 하나의 쓰기를 허용합니다.
    let config = Arc::new(RwLock::new(String::from("기본 설정")));
    let reader_config = Arc::clone(&config);
    let reader = thread::spawn(move || {
        let value = reader_config
            .read()
            .expect("RwLock 읽기 잠금이 오염되면 안 됩니다.");
        println!("RwLock 읽기 값: {value}");
    });

    {
        let mut value = config
            .write()
            .expect("RwLock 쓰기 잠금이 오염되면 안 됩니다.");
        value.push_str(" 변경됨");
    }
    reader
        .join()
        .expect("읽기 쓰레드가 정상 종료되어야 합니다.");

    // Barrier는 모든 쓰레드가 같은 지점에 도착할 때까지 기다립니다.
    let barrier = Arc::new(Barrier::new(3));
    let mut workers = Vec::new();

    for id in 1..=3 {
        let barrier = Arc::clone(&barrier);
        workers.push(thread::spawn(move || {
            println!("작업자 {id}: 준비 완료");
            barrier.wait();
            println!("작업자 {id}: 동시에 시작");
        }));
    }

    for worker in workers {
        worker
            .join()
            .expect("Barrier 작업이 정상 종료되어야 합니다.");
    }

    // Condvar는 특정 조건이 만족될 때까지 Mutex 보호 값의 잠금을 기다립니다.
    let ready = Arc::new((Mutex::new(false), Condvar::new()));
    let consumer_ready = Arc::clone(&ready);
    let consumer = thread::spawn(move || {
        let (lock, condition) = &*consumer_ready;
        let mut is_ready = lock
            .lock()
            .expect("Condvar의 Mutex가 잠금 중에 오염되면 안 됩니다.");
        while !*is_ready {
            is_ready = condition
                .wait(is_ready)
                .expect("Condvar 대기가 오염되면 안 됩니다.");
        }
        println!("Condvar 알림을 받은 소비자 실행");
    });

    let (lock, condition) = &*ready;
    {
        let mut is_ready = lock
            .lock()
            .expect("Condvar의 Mutex가 잠금 중에 오염되면 안 됩니다.");
        *is_ready = true;
    }
    condition.notify_one();
    consumer
        .join()
        .expect("Condvar 소비자가 정상 종료되어야 합니다.");

    // OnceLock은 모든 쓰레드가 공유하는 값을 한 번만 초기화합니다.
    let once_value = Arc::new(OnceLock::new());
    let mut workers = Vec::new();

    for id in 1..=3 {
        let once_value = Arc::clone(&once_value);
        workers.push(thread::spawn(move || {
            let value = once_value.get_or_init(|| {
                println!("한 번만 실행되는 초기화");
                id * 100
            });
            println!("쓰레드가 받은 초기화 값: {value}");
        }));
    }

    for worker in workers {
        worker
            .join()
            .expect("OnceLock 작업이 정상 종료되어야 합니다.");
    }
}
