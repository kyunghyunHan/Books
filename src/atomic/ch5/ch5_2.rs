use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Acquire;
use std::sync::atomic::Ordering::Release;
use std::sync::Mutex;
use std::{collections::VecDeque, sync::Condvar};
/*
MaybeUninit
- 본질적으로 Unsafe Option<T>의 기본버전

*/

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}
unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }
    pub unsafe fn send(&self, message: T) {
        (*self.message.get()).write(message);
        self.ready.store(true, Release);
    }
    pub fn is_ready(&self) -> bool {
        self.ready.load(Acquire)
    }
    pub unsafe fn receive(&self) -> T {
        (*self.message.get()).assume_init_read()
    }
}


pub fn main() {}
