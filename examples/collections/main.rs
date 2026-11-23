use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    // Vec는 같은 타입의 값을 순서대로 보관합니다.
    let mut scores = vec![("민수", 82), ("지현", 95), ("민수", 88)];
    scores.push(("서연", 91));
    println!("점수 목록: {scores:?}");

    // HashMap은 키를 통해 값을 조회합니다. 같은 키를 넣으면 기존 값이 교체됩니다.
    let mut latest_scores = HashMap::new();
    for (name, score) in &scores {
        latest_scores.insert(*name, *score);
    }
    println!("민수의 최근 점수: {:?}", latest_scores.get("민수"));

    // entry는 키의 존재 여부를 다시 조회하지 않고 값을 만들거나 갱신할 수 있습니다.
    let mut score_counts = HashMap::new();
    for (_, score) in &scores {
        *score_counts.entry(*score).or_insert(0) += 1;
    }
    println!("점수별 횟수: {score_counts:?}");

    // HashSet은 중복을 제거하고 포함 여부를 빠르게 확인할 때 유용합니다.
    let names: HashSet<_> = scores.iter().map(|(name, _)| *name).collect();
    println!("고유한 이름: {names:?}");

    // VecDeque는 앞과 뒤 양쪽에서 효율적으로 추가하거나 제거할 수 있습니다.
    let mut queue = VecDeque::from(["첫 번째", "두 번째"]);
    queue.push_back("세 번째");
    println!("큐에서 꺼낸 값: {:?}", queue.pop_front());
    println!("남은 큐: {queue:?}");
}
