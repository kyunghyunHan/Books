use std::collections::VecDeque;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn f1() {
    let queue = Mutex::new(VecDeque::new());
    thread::scope(|s| {
        //소비자 스레드
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                // dbg!(item);
                println!("{item}")
                
            } else {
                //큐가 비었다면 park상태로 들어감
                thread::park();
            }
        });
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            //추가할떄마다 unpark로 파킹상태를 해제
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
fn f2() {
    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| loop {
            let mut q = queue.lock().unwrap();
            let item = loop {
                if let Some(item) = q.pop_front() {
                    break item;
                } else {
                    q = not_empty.wait(q).unwrap();
                }
            };
            drop(q);
            dbg!(item);
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
pub fn main() {
    f2();
}
