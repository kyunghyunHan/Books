use atomic_wait::{wait, wake_one};
use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::AtomicU32,
};
// use libc::wait;
use crate::atomic::ch4::ch4_3::Guard;
use std::sync::atomic::Ordering::Acquire;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::Ordering::Release;

pub struct Mutex<T> {
    //0:잠금해제
    //1:잠금
    //2:대기중인 스레드가 있는 잠금 상태
    state: AtomicU32,
    value: UnsafeCell<T>,
}
unsafe impl<T> Sync for Mutex<T> where T: Send {}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}
unsafe impl<T> Sync for MutexGuard<'_, T> where T: Sync {}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            value: UnsafeCell::new(value),
        }
    }
    pub fn lock(&self) -> MutexGuard<T> {
        if self.state.compare_exchange(0, 1, Acquire, Relaxed).is_err() {
            // while self.state.swap(2, Acquire) != 0 {
            //     wait(&self.state, 2);
            // }
            lock_contended(&self.state)
        }
        // while self.state.swap(1, Acquire) == 1 {
        //     wait(&self.state, 1);
        // }
        MutexGuard { mutex: self }
    }
}

fn lock_contended(state: &AtomicU32) {
    let mut spin_count = 0;
    while state.load(Relaxed) == 1 && spin_count < 100 {
        spin_count += 1;
        std::hint::spin_loop();
    }

    if state.compare_exchange(0, 1, Acquire, Relaxed).is_ok() {
        return;
    }

    while state.swap(2, Acquire) != 0 {
        wait(state, 2);
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        if self.mutex.state.swap(0, Release) == 2 {
            wake_one(&self.mutex.state);
        }
        // self.mutex.state.store(0, Release);
        // wake_one(&self.mutex.state);
    }
}
pub fn main() {}
