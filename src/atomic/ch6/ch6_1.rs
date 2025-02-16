use std::sync::atomic::AtomicUsize;
use std::sync::Mutex;
use std::{collections::VecDeque, sync::Condvar};

struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}
pub fn main() {}
