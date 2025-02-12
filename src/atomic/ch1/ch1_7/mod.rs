use std::time::Duration;
use std::{sync::Mutex, thread};
fn f1() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
            });
        }
    });
    assert_eq!(n.into_inner().unwrap(), 1000); //into_inner=>뮤텍스의 소유권을 가져감
}
fn f2() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                thread::sleep(Duration::from_secs(1));
            });
        }
    });
    //뮤텍스는 한번에 한 스레드만 접근을 허용하기 때문에 10초정도 소요
    assert_eq!(n.into_inner().unwrap(), 1000);
}
fn f3() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                drop(guard); //스레드 대기 이전에 guard를 삭제
                thread::sleep(Duration::from_secs(1));
            });
        }
    })
}
pub fn main() {
    f3();
}
