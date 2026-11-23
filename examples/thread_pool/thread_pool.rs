use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};

// Box<dyn ...>은 서로 다른 클로저를 같은 타입으로 보관합니다.
// FnOnce는 한 번 실행할 작업, Send와 'static은 worker 쓰레드에 안전하게 넘길 조건입니다.
type Job = Box<dyn FnOnce() + Send + 'static>;
// 등록된 동작은 여러 번 호출하고 공유하므로 Arc와 Fn + Send + Sync를 사용합니다.
type Action = Arc<dyn Fn() + Send + Sync + 'static>;

/// 이 예제에서 등록된 명령 동작을 여러 worker 쓰레드에서 실행합니다.
pub struct ThreadPool {
    sender: Option<mpsc::Sender<Job>>,
    workers: Vec<JoinHandle<()>>,
    actions: HashMap<String, Action>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ThreadPoolError {
    UnknownCommand(String),
    Disconnected,
}

impl fmt::Display for ThreadPoolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownCommand(command) => {
                write!(formatter, "등록되지 않은 명령입니다: {command}")
            }
            Self::Disconnected => write!(formatter, "쓰레드 풀의 작업 채널이 닫혔습니다."),
        }
    }
}

impl Error for ThreadPoolError {}

impl ThreadPool {
    /// `size`개의 worker를 시작합니다. `size`는 1 이상이어야 합니다.
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "쓰레드 풀 크기는 1 이상이어야 합니다.");

        // std::sync::mpsc는 여러 생산자와 하나의 소비자를 위한 채널입니다.
        // Receiver를 여러 worker가 함께 쓰도록 Arc로 공유하고 Mutex로 한 번에 하나씩 받게 합니다.
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            // Arc::clone으로 공유 참조 횟수를 늘리고, move 클로저가 이를 worker로 소유권 이동합니다.
            let receiver = Arc::clone(&receiver);
            workers.push(thread::spawn(move || {
                loop {
                    // MutexGuard는 recv 결과를 얻은 뒤 작업 실행 전에 해제됩니다.
                    let job = match receiver.lock() {
                        Ok(receiver) => receiver.recv(),
                        Err(_) => break,
                    };

                    match job {
                        Ok(job) => job(),
                        // 모든 Sender가 drop되면 recv가 Err를 반환해 worker가 종료됩니다.
                        Err(_) => break,
                    }
                }
            }));
        }

        Self {
            sender: Some(sender),
            workers,
            actions: HashMap::new(),
        }
    }

    /// 명령 문자열에 실행할 동작을 등록합니다. 같은 이름을 다시 등록하면 동작이 바뀝니다.
    /// `impl Into<String>`은 String과 문자열 리터럴을 모두 받을 수 있게 합니다.
    pub fn register_command<F>(&mut self, command: impl Into<String>, action: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.actions.insert(command.into(), Arc::new(action));
    }

    /// 등록된 명령의 동작을 worker에게 요청합니다.
    pub fn request(&self, command: &str) -> Result<(), ThreadPoolError> {
        let action = self
            .actions
            .get(command)
            .cloned()
            .ok_or_else(|| ThreadPoolError::UnknownCommand(command.to_owned()))?;

        let sender = self.sender.as_ref().ok_or(ThreadPoolError::Disconnected)?;
        sender
            .send(Box::new(move || action()))
            .map_err(|_| ThreadPoolError::Disconnected)
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Sender가 사라지면 worker의 recv가 끝납니다. 채널에 남은 작업은 먼저 처리됩니다.
        self.sender.take();
        for worker in self.workers.drain(..) {
            if worker.join().is_err() {
                eprintln!("쓰레드 풀 worker가 작업 중 panic했습니다.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ThreadPool, ThreadPoolError};
    use std::sync::mpsc;
    use std::time::Duration;

    #[test]
    fn request_runs_registered_command() {
        let (sender, receiver) = mpsc::channel();
        let mut pool = ThreadPool::new(1);
        pool.register_command("signal", move || {
            sender
                .send(())
                .expect("테스트 수신자가 열려 있어야 합니다.");
        });

        pool.request("signal").expect("등록한 명령이어야 합니다.");

        receiver
            .recv_timeout(Duration::from_secs(1))
            .expect("요청한 동작이 실행되어야 합니다.");
    }

    #[test]
    fn request_reports_unknown_command() {
        let pool = ThreadPool::new(1);

        assert_eq!(
            pool.request("missing"),
            Err(ThreadPoolError::UnknownCommand("missing".to_owned()))
        );
    }
}
