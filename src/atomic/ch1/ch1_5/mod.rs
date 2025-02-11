use std::cell::{Cell, RefCell};
/*
cell은 단순히 T를 감싸고 있는 타입으로공유 레퍼런스로 값을 변경할수 있는 타입

*/
fn f(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        x();
    }
}
fn f2(v: &Cell<Vec<i32>>) {
    let mut v2 = v.take(); //Cell의 값을 빈 Vec로 대체
    v2.push(1);
    v.set(v2); //값이 변경된 Vec를 다시 입력
}
fn f3(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1); //Vec를 직접 수정 가능
}

/*
Rw Lock 은 읽기/쓰기 잠금 이라는 뜻으로 RefCell의 동시성 버전
RwLock<T>는 T를 가지고 있으면서 값이 몇번 대여되었는지를 내부적으로 추적
서로 상출되는 소유권대여가 발생하더라도 패닉을 일이크지 않음

RwLock의 값을 대여하는것을 lock이라고함


Mutex는 RwLock과 비슷하지만 더간단, 공유 혹은 독점적 소유권 대여가 몇번 일어났는지를 추적하는 대신 오직 독점적 소유권 대여만 허용
*/


/* 
Unsage Cell

내부 가변성을 위한 기본 구성 요소
*/
fn x() {
    println!("{}", "execution");
}
pub fn main() {
    let a = Cell::new(10);
    let b = Cell::new(10);

    f(&a, &a);
}
