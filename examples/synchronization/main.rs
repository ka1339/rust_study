use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier, Condvar, Mutex, OnceLock, RwLock};
use std::thread;

fn main() {
    // MutexGuard가 살아 있는 동안 잠금이 유지되고, 스코프를 벗어나면 자동으로 해제됩니다.
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

    // RwLock은 여러 읽기 잠금을 동시에 허용하지만, 쓰기 잠금은 단독으로만 얻을 수 있습니다.
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

    // Condvar는 알림 자체가 아니라 Mutex로 보호되는 조건을 기다립니다.
    let ready = Arc::new((Mutex::new(false), Condvar::new()));
    let consumer_ready = Arc::clone(&ready);
    let consumer = thread::spawn(move || {
        let (lock, condition) = &*consumer_ready;
        let mut is_ready = lock
            .lock()
            .expect("Condvar의 Mutex가 잠금 중에 오염되면 안 됩니다.");
        // Condvar는 드물게 조건을 만족하지 않아도 깨어날 수 있으므로 while로 조건을 다시 확인합니다.
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
    // 조건을 먼저 변경한 뒤 잠금을 놓고 알리면 소비자가 조건을 확인할 수 있습니다.
    condition.notify_one();
    consumer
        .join()
        .expect("Condvar 소비자가 정상 종료되어야 합니다.");

    // OnceLock은 여러 쓰레드가 동시에 요청해도 초기화 클로저를 한 번만 실행하고 그 값을 공유합니다.
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

    // Atomic은 단순한 숫자 갱신처럼 원자적으로 처리할 값에 Mutex 대신 사용할 수 있습니다.
    // SeqCst는 가장 강한 메모리 순서 보장을 제공하며, 복합 상태를 보호하는 용도는 아닙니다.
    let atomic_counter = Arc::new(AtomicUsize::new(0));
    let mut workers = Vec::new();

    for _ in 0..4 {
        let atomic_counter = Arc::clone(&atomic_counter);
        workers.push(thread::spawn(move || {
            atomic_counter.fetch_add(1, Ordering::SeqCst);
        }));
    }

    for worker in workers {
        worker
            .join()
            .expect("Atomic 작업 쓰레드가 정상 종료되어야 합니다.");
    }
    println!("Atomic 카운터: {}", atomic_counter.load(Ordering::SeqCst));
}
