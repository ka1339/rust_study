use std::num::ParseIntError;

fn average(samples: &[i32]) -> Option<f64> {
    if samples.is_empty() {
        return None;
    }

    let sum: i32 = samples.iter().sum();
    Some(f64::from(sum) / samples.len() as f64)
}

fn parse_limit(input: &str) -> Result<i32, ParseIntError> {
    input.parse()
}

fn main() -> Result<(), ParseIntError> {
    let samples = [21, 23, 24, 31];
    let warning_limit = parse_limit("30")?;

    // Option은 값이 있을 수도 없을 수도 있음을 표현합니다.
    match average(&samples) {
        Some(value) => println!("평균: {value:.1}"),
        None => println!("측정값이 없습니다."),
    }

    // Result는 성공(Ok) 또는 오류(Err)를 표현합니다.
    println!("경고 기준: {warning_limit}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::average;

    #[test]
    fn average_returns_none_for_empty_samples() {
        assert_eq!(average(&[]), None);
    }
}
