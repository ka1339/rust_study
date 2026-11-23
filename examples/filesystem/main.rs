use std::error::Error;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<(), Box<dyn Error>> {
    // PathBuf는 소유하는 경로이며, 경로 구분자를 직접 이어 붙이지 않아도 됩니다.
    let executable = std::env::current_exe()?;
    let executable_path = executable.as_path();
    println!("실행 파일 이름: {:?}", executable_path.file_name());
    println!("실행 파일 폴더: {:?}", executable_path.parent());

    // 파일 생성과 읽기를 시스템 임시 폴더에서 보여 주고, 실행 후 파일을 정리합니다.
    let unique_id = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let file_path = std::env::temp_dir().join(format!(
        "rust_study_{}_{}.txt",
        std::process::id(),
        unique_id
    ));

    fs::write(&file_path, "표준 라이브러리의 파일 시스템 예제\n")?;
    let contents = fs::read_to_string(&file_path)?;
    let metadata = fs::metadata(&file_path)?;
    println!("파일 내용: {}", contents.trim_end());
    println!("파일 크기: {} bytes", metadata.len());
    fs::remove_file(&file_path)?;

    Ok(())
}
