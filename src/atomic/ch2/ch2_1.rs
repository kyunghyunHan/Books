use std::os::unix::process;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::thread;
use std::{sync::atomic::AtomicUsize, time::Duration};

fn f1() {
    static STOP: AtomicBool = AtomicBool::new(false);

    //작업을 수행할 스레드를 생성힘

    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            some_work();
        }
    });

    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands:help,stop"),
            "stop" => break,
            cmd => println!("unknown command : {cmd:?}"),
        }
    }

    STOP.store(true, Relaxed);
    background_thread.join().unwrap();
}
fn f2() {
    let mut num_done = AtomicUsize::new(0);

    thread::scope(|s| {
        //백그라운드 스레드가 아이템 100개를 처리함
        s.spawn(|| {
            for i in 0..100 {
                process_item(i); //이작업에 시간이 소요된다고 가정
                num_done.store(i + 1, Relaxed);
            }
        });

        //메인 스레드는 매초 상태 업데이트 를 출력
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }

            println!("Working..{n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    });
    println!("Done");
}

fn f3() {
    let mut num_done = AtomicUsize::new(0);
    let main_thread = thread::current();

    thread::scope(|s| {
        //백그라운드 스레드가 아이템 100개를 처리함
        s.spawn(|| {
            for i in 0..100 {
                process_item(i); //이작업에 시간이 소요된다고 가정
                num_done.store(i + 1, Relaxed);
                main_thread.unpark(); //메인 스레드를 꺠움
            }
        });

        //메인 스레드는 매초 상태 업데이트 를 출력
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }

            println!("Working..{n}/100 done");
            thread::park_timeout(Duration::from_secs(1));
        }
    });
    println!("Done");
}

fn get_x() ->u64{
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Relaxed);
    if x == 0 {
        x = calculate_x();
        X.store(x, Relaxed);
    }
    x
}
pub fn main() {
    f3();
}

fn process_item(i: usize) {}
fn some_work() {
    println!("some work");
}
fn calculate_x() -> u64 {
    0
}
