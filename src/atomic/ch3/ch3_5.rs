use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::time::Duration;
use std::{
    sync::atomic::{AtomicBool, AtomicU32},
    thread,
};
static DATA: AtomicU32 = AtomicU32::new(0);
static READY: AtomicBool = AtomicBool::new(false);

pub fn main() {
    thread::spawn(|| {
        DATA.store(123, Relaxed);
        READY.store(true, Release);
    });

    while !READY.load(Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("waiting");
    }
    println!("{}", DATA.load(Relaxed));
}
