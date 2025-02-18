use std::cell::UnsafeCell;

use libc;

pub struct Mutex {
    // m: libc::pthread_mutex_t,
    // m:UnsafeCell<libc::pthread_mutex_t>
    m: Box<UnsafeCell<libc::pthread_mutex_t>>,
}

pub fn main() {
    // let m = Mutex::new();
    // let guard = m.lock();
    // std::mem::forget(guard);
}
