use std::cell::UnsafeCell;
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
    data: UnsafeCell<Option<T>>,
}

pub struct Arc<T> {
    weak: Weak<T>,
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
            weak: Weak {
                ptr: NonNull::from(Box::leak(Box::new(ArcData {
                    alloc_ref_count: AtomicUsize::new(1),
                    data_ref_count: AtomicUsize::new(1),
                    data: UnsafeCell::new(Some(data)),
                }))),
            },
        }
    }

    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        if arc.weak.data().alloc_ref_count.load(Relaxed) == 1 {
            fence(Acquire);
            //안전함 Arc가 단 하나만 존재하고 Weak는 한개도 없어서
            //현재 Arc가 독점적으로 접근가능
            let arcdata = unsafe { arc.weak.ptr.as_mut() };
            let option = arcdata.data.get_mut();
            //data를 가리키는 Arc가 있어서 패닉이 발생하지 않음
            let data = option.as_mut().unwrap();
            Some(data)
        } else {
            None
        }
    }
}




impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
}
impl<T> Deref for Arc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        let ptr = self.weak.data().data.get();
        //안전함 Arc가 data를 가리키고 있기 떄문에
        //data는 존재하고 공유될수 있음
        unsafe { (*ptr).as_ref().unwrap() }
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
impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        let weak = self.weak.clone();
        if weak.data().data_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {}
        Arc { weak }
    }
}
impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        //TODO 메모리 순서 추가하기
        if self.data().alloc_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        //TODO 메모리 순서 추가하기
        if self.weak.data().data_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            let ptr = self.weak.data().data.get();
            //안전함 data의 래퍼런스 카운터가 0이므로
            //이제 data의 접근 불가능
            unsafe {
                (*ptr) = None;
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
