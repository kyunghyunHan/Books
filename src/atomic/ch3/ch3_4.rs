use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn a() {
    X.fetch_add(5, Relaxed);
    X.fetch_add(10, Relaxed);
}
fn b() {
    let a = X.load(Relaxed);
    let b = X.load(Relaxed);
    let c = X.load(Relaxed);
    let d = X.load(Relaxed);
    println!("{a} {b} {c} {d}");
}
fn a1() {
    X.fetch_add(5, Relaxed);
    X.fetch_add(10, Relaxed);
}
fn a2() {
    X.fetch_add(5, Relaxed);
    X.fetch_add(10, Relaxed);
}

fn f1() {
    a();
    b();
}

fn f2() {
    let a = thread::spawn(|| {
        let x = X.load(Relaxed);
        Y.store(x, Relaxed);
    });
    let b = thread::spawn(|| {
        let y = Y.load(Relaxed);
        X.store(y, Relaxed);
    });

    a.join().unwrap();
    b.join().unwrap();
    println!("{}", X.load(Relaxed));
    assert_eq!(X.load(Relaxed), 0); //실패할수도 있음
    assert_eq!(Y.load(Relaxed), 0); //실패할수도 있음
}
pub fn main() {
    f2();
}
