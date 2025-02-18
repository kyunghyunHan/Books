use atomic_wait::{wait, wake_all, wake_one};
use std::sync::atomic::AtomicUsize;
use std::time::Duration;
use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::AtomicU32,
    time::Instant,
};
use std::{thread, u32};
// use libc::wait;
use std::sync::atomic::Ordering::Acquire;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::Ordering::Release;

pub struct RwLock<T> {
    //읽기 잠금 수 에 2를 곱하고
    state: AtomicU32,
    writer_wake_counter: AtomicU32, //추가
    value: UnsafeCell<T>,
}
pub struct ReadGuard<'a, T> {
    rwlock: &'a RwLock<T>,
}
pub struct WriteGuard<'a, T> {
    rwlock: &'a RwLock<T>,
}

unsafe impl<T> Sync for RwLock<T> where T: Send + Sync {}

impl<T> RwLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            writer_wake_counter: AtomicU32::new(0), //추가
            value: UnsafeCell::new(value),
        }
    }

    pub fn read(&self) -> ReadGuard<T> {
        let mut s = self.state.load(Relaxed);
        loop {
            if s % 2 == 0 {
                //짝수
                assert!(s < u32::MAX - 2, "too many readers");
                match self.state.compare_exchange_weak(s, s + 2, Acquire, Relaxed) {
                    Ok(_) => return ReadGuard { rwlock: self },
                    Err(e) => s = e,
                }
            }
            if s % 2 == 1 {
                //홀수
                wait(&self.state, s);
                s = self.state.load(Relaxed);
            }

            // if s < u32::MAX {
            //     assert!(s != u32::MAX - 1, "too many readers");
            //     match self.state.compare_exchange_weak(s, s + 1, Acquire, Relaxed) {
            //         Ok(_) => return ReadGuard { rwlock: self },
            //         Err(e) => s = e,
            //     }
            // }
            // if s == u32::MAX {
            //     wait(&self.state, u32::MAX);
            //     s = self.state.load(Relaxed);
            // }
        }
    }

    pub fn write(&self) -> WriteGuard<T> {
        // while let Err(s) = self.state.compare_exchange(0, u32::MAX, Acquire, Relaxed) {
        //     wait(&self.state, s);
        // }

        // while self
        //     .state
        //     .compare_exchange(0, u32::MAX, Acquire, Relaxed)
        //     .is_err()
        // {
        //     let w = self.writer_wake_counter.load(Acquire);
        //     if self.state.load(Relaxed) != 0 {
        //         wait(&self.writer_wake_counter, w);
        //     }
        // }
        // WriteGuard { rwlock: self }

        let mut s = self.state.load(Relaxed);
        loop {
            if s <= 1 {
                match self.state.compare_exchange(s, u32::MAX, Acquire, Relaxed) {
                    Ok(_) => return WriteGuard { rwlock: self },
                    Err(e) => {
                        s = e;
                        continue;
                    }
                }
            }
            if s % 2 == 0 {
                match self.state.compare_exchange(s, s + 1, Relaxed, Relaxed) {
                    Ok(_) => {}
                    Err(e) => {
                        s = e;
                        continue;
                    }
                }
            }
            let w = self.writer_wake_counter.load(Acquire);
            s = self.state.load(Relaxed);
            if s >= 2 {
                wait(&self.writer_wake_counter, w);
                s = self.state.load(Relaxed);
            }
        }
    }
}
impl<T> Deref for WriteGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.rwlock.value.get() }
    }
}
impl<T> DerefMut for WriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.rwlock.value.get() }
    }
}

impl<T> Deref for ReadGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.rwlock.value.get() }
    }
}

impl<T> Drop for ReadGuard<'_, T> {
    fn drop(&mut self) {
        // if self.rwlock.state.fetch_sub(1, Release) == 1 {
        //     wake_one(&self.rwlock.state);
        // }
        // if self.rwlock.state.fetch_sub(1, Release) == 1 {
        //     self.rwlock.writer_wake_counter.fetch_add(1, Release); //추가

        //     wake_one(&self.rwlock.writer_wake_counter); //변경
        // }

        if self.rwlock.state.fetch_sub(2, Release) == 3 {
            self.rwlock.writer_wake_counter.fetch_add(1, Release);
            wake_one(&self.rwlock.writer_wake_counter);
        }
    }
}
impl<T> Drop for WriteGuard<'_, T> {
    fn drop(&mut self) {
        self.rwlock.state.store(0, Release);
        self.rwlock.writer_wake_counter.fetch_add(1, Release); //추가
        wake_one(&self.rwlock.writer_wake_counter); //추가
        wake_all(&self.rwlock.state);
    }
}
pub fn main() {}
