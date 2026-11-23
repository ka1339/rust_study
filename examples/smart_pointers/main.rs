use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn sum_list(list: &List) -> i32 {
    match list {
        List::Cons(value, next) => value + sum_list(next),
        List::Nil => 0,
    }
}

fn main() {
    // Box는 값을 힙에 저장합니다. 재귀 타입은 Box로 크기를 고정할 수 있습니다.
    let list = Box::new(List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil)))));
    println!("Box로 만든 리스트: {list:?}");
    println!("리스트 원소의 합: {}", sum_list(&list));

    // Rc는 단일 스레드에서 여러 소유자가 값을 공유하게 합니다. 여러 스레드에서 공유할 때는 Arc를 사용합니다.
    // RefCell은 실행 시점에 대여 규칙을 검사하며, 동시에 여러 가변 대여를 시도하면 panic이 발생합니다.
    let shared_notes = Rc::new(RefCell::new(vec![String::from("시작")]));
    let first_owner = Rc::clone(&shared_notes);
    let second_owner = Rc::clone(&shared_notes);
    first_owner.borrow_mut().push(String::from("첫 번째 수정"));

    println!("공유된 값: {:?}", second_owner.borrow());
    println!("공유 소유자 수: {}", Rc::strong_count(&shared_notes));
}
