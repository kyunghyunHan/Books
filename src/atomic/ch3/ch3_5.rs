use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::time::Duration;
use std::{
    sync::atomic::{AtomicBool, AtomicU32},
    thread,
};
static mut DATA: u64 = 0;

// static DATA: AtomicU32 = AtomicU32::new(0);
static READY: AtomicBool = AtomicBool::new(false);

pub fn main() {
    thread::spawn(|| {
        //안전함:아직 Ready 플래그를 성정하지 않아서
        // DATA.store(123, Relaxed);
        unsafe { DATA = 123 }
        READY.store(true, Release); //이 저장 연산 이전의 모든 값들은
    });

    while !READY.load(Acquire) {
        //여기서 true를 불러온 이후에 확인 가능
        thread::sleep(Duration::from_millis(100));
        println!("waiting");
    }
    //안전함:READY가 설정되어서 어떤 스레드도 DATA를 변경하지 않음
    println!("{}", unsafe {
        {
            DATA
        }
    });

    // println!("{}", DATA.load(Relaxed));
}
