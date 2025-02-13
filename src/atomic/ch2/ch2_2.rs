use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicI32, AtomicUsize};
use std::thread;
use std::time::Duration;
fn f1() {
    let a = AtomicI32::new(100);
    let b = a.fetch_add(23, Relaxed);
    let c = a.load(Relaxed);
    assert_eq!(b, 100);
    assert_eq!(c, 123);
}

fn f2() {
    let num_done = &AtomicUsize::new(0);
    thread::scope(|s| {
        //아이템 100개를 스레드 4개가 각각 25개씩 처리
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    process_item(t * 25 + 1); //이 작업에 시간이 소요된다고 가정
                    num_done.fetch_add(1, Relaxed);
                }
            });
        }
        //메인 스레드는 매초 상태 업데이트로 출력
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working..{n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    })
}
pub fn main() {
    f2();
}
fn process_item(i: usize) {}
