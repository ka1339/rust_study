use std::io::{self, BufRead, Cursor, Write};

fn main() -> io::Result<()> {
    // Cursor는 문자열을 읽기 가능한 입력처럼 다루게 해 주므로 파일 없이 입력 처리를 테스트할 때 유용합니다.
    let input = Cursor::new("사과\n배\n포도\n");
    // lines()의 각 항목은 Result이므로 collect::<Result<_, _>>()는 한 줄이라도 실패하면 오류를 돌려줍니다.
    let lines: Vec<String> = input.lines().collect::<Result<_, _>>()?;
    println!("읽은 줄: {lines:?}");

    // Write 인터페이스는 Vec<u8> 같은 메모리 버퍼에도 쓸 수 있습니다.
    let mut output = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        writeln!(&mut output, "{}. {line}", index + 1)?;
    }
    print!("{}", String::from_utf8_lossy(&output));

    Ok(())
}
