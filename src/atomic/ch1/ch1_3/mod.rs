use std::rc::Rc;
use std::thread;
/*
static: 정적 변수 선언을 위한 키워드 프로그램이 시작댈떄터 존재
'static: 프로그램 전체 수명 동안 유효한 참조를 나타내는 라이프타임 프로그램이 종료될떄까지 유지댄다는 의미
*/

fn statics() {
    //static 변수는 프로그램 자체가 소유권을 가지기 떄문에 어떤 스레드보다 더 오래존재
    //어떤 스레드여도 값을 참조 가능
    static x: [i32; 3] = [1, 2, 3];
    thread::spawn(|| dbg!(&x));
    thread::spawn(|| dbg!(&x));
}
fn leaking() {
    /*누수
    Box의 소유권을 해제하고 이값이 드랍되지 않도록 할수 있음Box는 프르그램이 종료될떄까지 존재
    이렇게 되면 메모리가 누수되는 단점이 있음
     */
    let y: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    thread::spawn(move || dbg!(y));
    thread::spawn(move || dbg!(y));
}
fn rc() {
    /*
    스레드 사이에서 공유된 데이터가 확실히 드랍되고 할당된 메모리도 해제되게 하려면 해당 데이터의 소유권을 포기해서는 안댐
    대신 소유권을 공유하면 가능
    원본 Rc와 복제된 Rc 모두 같은 메모리에 할당된 값을 참조 => 소유권 공유

    스레드 안정성이 보장되지 않는 타입
    여러개의 스레드가 특정 값에 대해 Rc를 사용한다면 각 스레드에서 레퍼런스 카운ㅌ를 동시에 변경할수 있는 가능성이 있고 이상한 결과를 발생
    */

    let a = Rc::new([1, 2, 3]);
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr()); //같은 메모리를 가짐
}

fn arc() {}
pub fn main() {
    // statics();
    // leaking();
    rc();
    arc();
}
