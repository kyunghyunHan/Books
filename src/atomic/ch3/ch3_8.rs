use std::sync::atomic::fence;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release, SeqCst};
use std::time::Duration;
use std::{
    sync::atomic::{AtomicBool, AtomicU32},
    thread,
};
static mut DATA: [u64; 10] = [0; 10];
const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

static mut S: String = String::new();
pub fn main() {
    for i in 0..10 {
        thread::spawn(move || {
            let data = some_calculation(i);
            unsafe { DATA[i] = data };
            READY[i].store(true, Release);
        });
    }

    thread::sleep(Duration::from_millis(500));
    let ready: [bool; 10] = std::array::from_fn(|i| READY[i].load(Relaxed));
    if ready.contains(&true) {
        fence(Acquire);
        for i in 0..10 {
            if ready[i] {
                println!("data{i} = {} ", unsafe { DATA[i] });
            }
        }
    }
}
fn some_calculation(i: usize) -> u64 {
    1
}
