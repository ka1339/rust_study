use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn main() {
    // thread::spawn은 새 OS 쓰레드를 만들고, join으로 종료까지 기다립니다.
    let numbers = vec![1, 2, 3, 4, 5];
    let producer = thread::spawn({
        let numbers = numbers.clone();
        move || {
            let sum: i32 = numbers.iter().sum();
            println!("생산자 쓰레드의 합계: {sum}");
            sum
        }
    });

    println!("주 쓰레드에서 계속 다른 작업을 수행합니다.");
    let sum = producer
        .join()
        .expect("계산 쓰레드가 정상 종료되어야 합니다.");
    println!("join으로 받은 결과: {sum}");

    // mpsc 채널은 여러 송신자에서 한 수신자에게 값을 전달합니다.
    let (sender, receiver) = mpsc::channel();
    let child_sender = sender.clone();
    let sender_thread = thread::spawn(move || {
        for value in 1..=3 {
            child_sender
                .send(value)
                .expect("수신자가 열려 있어야 합니다.");
        }
    });
    drop(sender);

    println!("채널로 받은 값:");
    for value in receiver {
        println!("  {value}");
    }
    sender_thread
        .join()
        .expect("송신 쓰레드가 정상 종료되어야 합니다.");

    // Arc는 여러 쓰레드가 소유권을 공유하게 하고, Mutex는 한 번에 하나의 쓰레드만 값을 수정하게 합니다.
    let shared_total = Arc::new(Mutex::new(0));
    let mut workers = Vec::new();

    for offset in 0..4 {
        let shared_total = Arc::clone(&shared_total);
        workers.push(thread::spawn(move || {
            let mut total = shared_total
                .lock()
                .expect("Mutex가 잠금 중에 오염되면 안 됩니다.");
            *total += (offset + 1) * 10;
        }));
    }

    for worker in workers {
        worker
            .join()
            .expect("작업 쓰레드가 정상 종료되어야 합니다.");
    }
    println!(
        "공유된 최종 합계: {}",
        *shared_total
            .lock()
            .expect("Mutex가 잠금 중에 오염되면 안 됩니다.")
    );
}
