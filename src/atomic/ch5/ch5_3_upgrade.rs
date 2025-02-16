use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Acquire;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::Ordering::Release;

use std::sync::Mutex;
use std::thread;
use std::{collections::VecDeque, sync::Condvar};
/*
MaybeUninit
- 본질적으로 Unsafe Option<T>의 기본버전

*/

const EMPTY: u8 = 0;
const WRITING: u8 = 1;
const READY: u8 = 2;
const READING: u8 = 3;

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    in_use: AtomicBool,
    ready: AtomicBool,
}
unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            in_use: AtomicBool::new(false),
            ready: AtomicBool::new(false),
        }
    }
    pub fn send(&self, message: T) {
        if self.in_use.swap(true, Relaxed) {
            panic!("can't send more than one message");
        }
        unsafe {
            (*self.message.get()).write(message);
        }
        self.ready.store(true, Release);
    }
    pub fn is_ready(&self) -> bool {
        self.ready.load(Relaxed)
    }
    pub fn receive(&self) -> T {
        // if !self.ready.load(Acquire) {
        //     panic!("No Message avaiolable");
        // }
        if !self.ready.swap(false, Acquire) {
            panic!("No Message avaiolable");
        }
        //안전함:ready 플래그를 확인하고 초기화
        unsafe { (*self.message.get()).assume_init_read() }
    }
}
impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe {
                self.message.get_mut().assume_init_drop();
            }
        }
    }
}
pub fn main() {
    let channel = Channel::new();
    let t = thread::current();
    thread::scope(|s| {
        s.spawn(|| {
            channel.send("hello world!");
            t.unpark();
        });
        while !channel.is_ready() {
            thread::park();
        }
        assert_eq!(channel.receive(), "hello world!");
    });
}
