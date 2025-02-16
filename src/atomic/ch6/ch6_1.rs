use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::Ordering::Release;
use std::sync::Mutex;
use std::{collections::VecDeque, sync::Condvar};
struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                ref_count: AtomicUsize::new(1),
                data,
            }))),
        }
    }
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
}
impl<T> Deref for Arc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.data().data
    }
}
impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        //TODO 오버 플로 처리
        self.data().ref_count.fetch_add(1, Relaxed);
        Arc { ptr: self.ptr }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        //TODO 메모리 순서 추가하기
        if self.data().ref_count.fetch_sub(1, Release) == 1 {
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
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

pub fn main() {}
