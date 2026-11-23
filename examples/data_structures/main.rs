use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Stack<T> {
    values: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self { values: Vec::new() }
    }

    fn push(&mut self, value: T) {
        self.values.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.values.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.values.last()
    }
}

#[derive(Debug)]
struct Queue<T> {
    values: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            values: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, value: T) {
        self.values.push_back(value);
    }

    fn dequeue(&mut self) -> Option<T> {
        self.values.pop_front()
    }
}

fn main() {
    // Stack<T>과 Queue<T>는 임의의 원소 타입 T를 저장하며, 비어 있을 때 꺼내기는 None을 반환합니다.
    // Stack은 후입선출(LIFO)이고 Queue는 선입선출(FIFO)입니다.
    let mut stack = Stack::new();
    stack.push("첫 번째");
    stack.push("두 번째");
    println!("스택 최상단: {:?}", stack.peek());
    println!("스택에서 꺼냄: {:?}", stack.pop());

    let mut queue = Queue::new();
    queue.enqueue("고객 A");
    queue.enqueue("고객 B");
    println!("큐에서 꺼냄: {:?}", queue.dequeue());
    println!("큐에 남은 값: {:?}", queue.values);

    // Vec는 인덱스로 빠르게 접근하고, HashMap은 키로 값을 찾습니다.
    let scores = vec![82, 95, 88];
    let mut names = HashMap::new();
    names.insert("민수", 82);
    names.insert("지현", 95);
    println!("첫 번째 점수: {}", scores[0]);
    println!("지현의 점수: {:?}", names.get("지현"));

    // HashSet은 중복 없는 값의 집합이고, BTreeMap은 정렬된 키를 보관합니다.
    let unique: HashSet<_> = [1, 2, 2, 3].into_iter().collect();
    let sorted = BTreeMap::from([("감", 3), ("배", 2), ("사과", 5)]);
    println!("중복이 제거된 집합: {unique:?}");
    println!("키순으로 정렬된 맵:");
    for (name, count) in &sorted {
        println!("  {name}: {count}");
    }

    // BinaryHeap은 가장 큰 값을 pop으로 먼저 꺼냅니다.
    let mut heap = BinaryHeap::from([3, 1, 7, 5]);
    println!("최댓값 힙에서 꺼냄: {:?}", heap.pop());
    println!("힙에 남은 값: {heap:?}");
}
