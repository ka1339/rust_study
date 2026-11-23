mod thread_pool;
use std::io::{self, BufRead};
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;
use thread_pool::ThreadPool;

fn main() -> io::Result<()> {
    let mut pool = ThreadPool::new(3);

    pool.register_command("work", || {
        println!("작업 시작");
        thread::sleep(Duration::from_secs(1));
        println!("작업 완료");
    });

    // Arc로 풀을 공유해 입력을 기다리는 동안에도 별도 쓰레드가 명령을 요청할 수 있습니다.
    let pool = Arc::new(pool);
    let requester_pool = Arc::clone(&pool);
    let (stop_sender, stop_receiver) = mpsc::channel();
    let requester = thread::spawn(move || {
        loop {
            // std 채널의 시간 제한 수신으로 주기 요청과 종료 알림을 함께 기다립니다.
            match stop_receiver.recv_timeout(Duration::from_secs(2)) {
                Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    if let Err(error) = requester_pool.request("work") {
                        eprintln!("주기 요청 실패: {error}");
                        break;
                    }
                }
            }
        }
    });

    println!("2초마다 work 명령을 요청합니다. 종료하려면 터미널에 exit를 입력하세요.");
    // stdin.lock()은 표준 입력을 잠근 뒤, lines()로 한 줄씩 읽는 반복자를 만듭니다.
    let mut input_error = None;
    for line in io::stdin().lock().lines() {
        match line {
            Ok(command) if command.trim() == "exit" => {
                println!("종료 명령을 받았습니다. 남은 작업을 마친 뒤 종료합니다.");
                break;
            }
            Ok(_) => println!("종료하려면 exit를 입력하세요."),
            Err(error) => {
                input_error = Some(error);
                break;
            }
        }
    }

    // 종료 채널을 닫으면 주기 요청 쓰레드가 대기 중이어도 깨어나 종료합니다.
    drop(stop_sender);
    if requester.join().is_err() {
        eprintln!("주기 요청 쓰레드가 비정상 종료되었습니다.");
    }

    // 풀의 마지막 참조를 해제하면 남은 작업을 마친 뒤 모든 worker가 종료됩니다.
    drop(pool);
    println!("쓰레드 풀이 종료되었습니다.");
    match input_error {
        Some(error) => Err(error),
        None => Ok(()),
    }
}
