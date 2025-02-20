use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicI32, AtomicU64, AtomicUsize};
use std::thread;
use std::time::{Duration, Instant};
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
    });
    println!("Done!");
}

fn f3() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);
    thread::scope(|s| {
        //아이템 100개를 스레드 4개가 각각 25개씩 처리
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    process_item(t * 25 + 1); //이 작업에 시간이 소요된다고 가정
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Relaxed);
                    total_time.fetch_add(time_taken, Relaxed);
                    max_time.fetch_max(time_taken, Relaxed);
                }
            });
        }
        //메인 스레드는 매초 상태 업데이트로 출력
        loop {
            let total_time = Duration::from_micros(total_time.load(Relaxed));
            let max_time = Duration::from_micros(max_time.load(Relaxed));
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            if n == 0 {
                println!("Working.. nothing done yet");
            } else {
                println!(
                    "Working.. {n}/100 done, {:?} average, {:?} peak",
                    total_time / n as u32,
                    max_time
                )
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
    println!("Done!");
}
pub fn main() {
    f3();
}
fn process_item(i: usize) {}
