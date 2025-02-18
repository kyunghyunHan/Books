use std::cell::UnsafeCell;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::fence;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Acquire;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::Ordering::Release;

use std::sync::Mutex;
use std::{collections::VecDeque, sync::Condvar};
struct ArcData<T> {
    //Arc의 개수
    data_ref_count: AtomicUsize,
    //Arc와 Weak으 개수 합계
    alloc_ref_count: AtomicUsize,
    //워크 포인터만 남은 경우 data는 None상태가 댐
    data: UnsafeCell<ManuallyDrop<T>>,
}

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}
unsafe impl<T: Send + Sync> Send for Weak<T> {}
unsafe impl<T: Send + Sync> Sync for Weak<T> {}
unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                alloc_ref_count: AtomicUsize::new(1),
                data_ref_count: AtomicUsize::new(1),
                data: UnsafeCell::new(ManuallyDrop::new(data)),
            }))),
        }
    }
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        if arc
            .data()
            .alloc_ref_count
            .compare_exchange(1, usize::MAX, Acquire, Relaxed)
            .is_err()
        {
            return None;
        }

        let is_unique = arc.data().data_ref_count.load(Relaxed) == 1;
        arc.data().alloc_ref_count.store(1, Release);
        if !is_unique {
            return None;
        }

        fence(Acquire);
        unsafe { Some(&mut *arc.data().data.get()) }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        //안전함 Arc가 data를 가리키고 있기 떄문에
        //data는 존재하고 공유될수 있음
        unsafe { &*self.data().data.get() }
    }
}
impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        if self.data().data_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Arc { ptr: self.ptr }
    }
}
impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn upgrade(&self) -> Option<Arc<T>> {
        let mut n = self.data().data_ref_count.load(Relaxed);
        loop {
            if n == 0 {
                return None;
            }
            assert!(n < usize::MAX);
            if let Err(e) =
                self.data()
                    .data_ref_count
                    .compare_exchange_weak(n, n + 1, Relaxed, Relaxed)
            {
                n = e;
                continue;
            }
            return Some(Arc { ptr: self.ptr });
        }
    }
    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        if arc
            .data()
            .alloc_ref_count
            .compare_exchange(1, usize::MAX, Acquire, Relaxed)
            .is_err()
        {
            return None;
        }

        let is_unique = arc.data().data_ref_count.load(Relaxed) == 1;
        arc.data().alloc_ref_count.store(1, Release);
        if !is_unique {
            return None;
        }

        fence(Acquire);
        unsafe { Some(&mut *arc.data().data.get()) }
    }
    pub fn downgrade(arc: &Self) -> Weak<T> {
        let mut n = arc.data().alloc_ref_count.load(Relaxed);
        loop {
            if n == usize::MAX {
                std::hint::spin_loop();
                n = arc.data().alloc_ref_count.load(Relaxed);
                continue;
            }
            assert!(n < usize::MAX - 1);
            if let Err(e) =
                arc.data()
                    .alloc_ref_count
                    .compare_exchange_weak(n, n + 1, Acquire, Relaxed)
            {
                n = e;
                continue;
            }
            return Weak { ptr: arc.ptr };
        }
    }
}
impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        if self.data().alloc_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Weak { ptr: self.ptr }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        if self.data().alloc_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe { drop(Box::from_raw(self.ptr.as_ptr())) }
        }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().alloc_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe {
                ManuallyDrop::drop(&mut *self.data().data.get());
            }
            drop(Weak { ptr: self.ptr });
        }
    }
}

#[test]

fn test() {
    static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

    struct DetectDrop;

    impl Drop for DetectDrop {
        fn drop(&mut self) {
            NUM_DROPS.fetch_add(1, Relaxed);
        }
    }

    //문자열과 DetectDrop를 포함한 객체를 공유하는 두 Arc생성
    //이제 객체가 언제 삭제되는지 알수 있음
    let x = Arc::new(("hello", DetectDrop));
    let y = x.clone();

    //x를 다른 스레드로 보내고 그곳에서 사용
    let t = std::thread::spawn(move || assert_eq!(x.0, "hello"));

    //y는 여전히 여기서 사용가능
    assert_eq!(y.0, "hello");

    //스레드가 끝나길 기다림
    t.join().unwrap();

    //이제 Arc x 는 메모리에서 삭제
    //아직 y가 존재하므로 객체도 존재
    assert_eq!(NUM_DROPS.load(Relaxed), 0);

    //남아있는 Arc y도 삭제
    drop(y);

    //y도 삭제되므로 객체도 삯제
    assert_eq!(NUM_DROPS.load(Relaxed), 1);
}
struct Something {}
pub fn main() {}
// fn annoying(mut arc: Arc<Something>) {
//     let my_arc = Arc::new(());

//     loop {
//         let weak = Arc::downgrade(&arc);
//         drop(arc);
//         println!("I have no Arc");
//         arc = weak.ungrad().unwrap();
//         drop(weak);
//         println!("i have no Weak");
//     }
// }
