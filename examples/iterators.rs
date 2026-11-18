fn main() {
    let samples = vec![21, 23, 24, 31];

    // iter()로 빌려 읽고 filter/map으로 변환한 결과를 수집합니다.
    let high_samples: Vec<i32> = samples
        .iter()
        .filter(|&&sample| sample > 23)
        .map(|&sample| sample * 2)
        .collect();

    // 반복자는 합계나 개수처럼 결과 하나만 구할 때도 사용할 수 있습니다.
    let total: i32 = samples.iter().sum();
    let high_sample_count = samples.iter().filter(|&&sample| sample > 30).count();

    println!("원본 측정값: {samples:?}");
    println!("기준 초과 값에 2를 곱한 결과: {high_samples:?}");
    println!("합계: {total}");
    println!("30 초과 횟수: {high_sample_count}");
}
