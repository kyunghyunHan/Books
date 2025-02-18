use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::thread;
struct X {
    handle: i32,
    _not_sync: PhantomData<Cell<()>>,
}

unsafe impl Send for X {}
unsafe impl Sync for X {}
pub fn main() {
    let a = Rc::new(123);
    thread::spawn(move || {
        //애러
        // dbg!(a); Arc와 달리 Send트레이트를 구현하지 않기떄문에 오류
    });
}
