use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Acquire;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::Ordering::Release;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::Thread;
use std::{collections::VecDeque, sync::Condvar};
/*
MaybeUninit
- 본질적으로 Unsafe Option<T>의 기본버전

*/
struct Channel<T> {
    //더이상 public이 아님
    message: UnsafeCell<MaybeUninit<T>>,

    ready: AtomicBool,
}
pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
    receiving_thread: Thread, //추가
}
pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
    _no_send: PhantomData<*const ()>, //추가
}
impl<T> Sender<'_, T> {
    pub fn send(self, message: T) {
        unsafe {
            (*self.channel.message.get()).write(message);
        }
        self.channel.ready.store(true, Release);
        self.receiving_thread.unpark(); //추가
    }
}
impl<T> Receiver<'_, T> {
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Relaxed)
    }
    pub fn receive(self) -> T {
        while !self.channel.ready.swap(false, Acquire) {
            thread::park();
        }
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }
    pub fn spilt<'a>(&'a mut self) -> (Sender<'a, T>, Receiver<'a, T>) {
        *self = Self::new();
        (
            Sender {
                channel: self,
                receiving_thread: thread::current(),
            },
            Receiver {
                channel: self,
                _no_send: PhantomData,
            },
        )
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
    let mut channel = Channel::new();

    thread::scope(|s| {
        let (sender, receiver) = channel.spilt();
        let t = thread::current();
        s.spawn(move || {
            sender.send("hello world!");
            t.unpark();
        });
        while !receiver.is_ready() {
            thread::park();
        }
        assert_eq!(receiver.receive(), "hello world!");
    });
}
