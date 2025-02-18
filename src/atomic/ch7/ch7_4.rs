use std::hint::black_box;
use std::sync::atomic::Ordering::Acquire;
use std::sync::atomic::Ordering::Release;

use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{compiler_fence, AtomicBool, AtomicUsize};
use std::thread;
use std::{sync::atomic::AtomicU64, time::Instant};

pub fn main() {
    let locked = AtomicBool::new(false);
    let counter = AtomicUsize::new(0);

    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _i in 0..1_000_000 {
                    while locked.swap(true, Relaxed) {}
                    compiler_fence(Acquire);

                    let old = counter.load(Relaxed);
                    let new = old + 1;
                    counter.store(new, Relaxed);
                    compiler_fence(Release);
                    locked.store(false, Relaxed);
                }
            });
        }
    });
    println!("{}", counter.into_inner());
}
