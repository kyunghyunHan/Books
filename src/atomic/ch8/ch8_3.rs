use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

use libc::wait;
#[cfg(not(any(target_os = "linux", target_os = "macos")))]

compile_error!("Linux or macOS only. Sorry!");
const SYS_FUTEX: i64 = 202; // x86_64 기준

// pub fn wait(a: &AtomicU32, expected: u32) {
//     unsafe {
//         libc::syscall(
//             libc::SYS_futex
//             a as *const AtomicU32,
//             libc::FUTEX_WAIT,
//             expected,
//             std::ptr::null::<libc::timespec>(),
//         );
//     }
// }

pub fn wake_one(a: AtomicU32) {
    //     unsafe {
    //         libc::syscall(
    //             libc::SYS_futex
    //             a as *const AtomicU32,
    //             libc::FUTEX_WAIT,
    //             expected,
    //             1//
    //         );
    //     }
}

pub fn main() {
    let a = AtomicU32::new(0);
    thread::scope(|s| {
        s.spawn(|| {
            thread::sleep(Duration::from_secs(3));
            a.store(1, Relaxed);
            // wake_one(&a);
        });
        println!("Watting");
        while a.load(Relaxed) == 0 {
            // wait(&a, 0);
        }
        println!("Done!");
    });
}
