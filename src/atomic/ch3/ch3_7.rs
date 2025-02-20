use std::sync::atomic::Ordering::{Acquire, Relaxed, Release, SeqCst};
use std::time::Duration;
use std::{
    sync::atomic::{AtomicBool, AtomicU32},
    thread,
};

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();
pub fn main() {
    let a = thread::spawn(|| {
        A.store(true, SeqCst);
        if !B.load(SeqCst) {
            unsafe {
                S.push('!');
            }
        }
    });

    let b = thread::spawn(|| {
        B.store(true, SeqCst);
        if !A.load(SeqCst) {
            unsafe {
                S.push('!');
            }
        }
    });

    a.join().unwrap();
    b.join().unwrap();
}
