fn main() {
    // String은 소유하는 UTF-8 문자열이며 내용을 변경할 수 있습니다.
    let mut message = String::from("안녕하세요");
    message.push_str(", Rust");
    message.push('!');
    println!("{message}");

    // format!은 원본 문자열을 소비하지 않고 새 문자열을 만듭니다.
    let report = format!("문자열: {message}, 글자 수: {}", message.chars().count());
    println!("{report}");

    // 문자열 슬라이스는 UTF-8 문자 경계가 중요하므로 문자 단위 처리는 chars()를 사용합니다.
    let words: Vec<_> = message.split_whitespace().collect();
    let replaced = message.replace("Rust", "표준 라이브러리");
    println!("단어: {words:?}");
    println!("치환 결과: {replaced}");

    let padded = "  공백 제거  ";
    println!("trim 결과: '{}'", padded.trim());
}
