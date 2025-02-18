use std::hint::black_box;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::{sync::atomic::AtomicU64, time::Instant};
#[repr(align(64))] //이 구조체는 64비트 크기로정렬
struct Aligned(AtomicU64);
static A: AtomicU64 = AtomicU64::new(0);
static B: [AtomicU64; 3] = [
    (AtomicU64::new(0)),
    (AtomicU64::new(0)),
    (AtomicU64::new(0)),
];
static C: [Aligned; 3] = [
    Aligned(AtomicU64::new(0)),
    Aligned(AtomicU64::new(0)),
    Aligned(AtomicU64::new(0)),
];

fn f1() {
    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        A.load(Relaxed);
    }
    println!("{:?}", start.elapsed());
}

fn f2() {
    black_box(&A); //추가됨
    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(A.load(Relaxed));
    }
    println!("{:?}", start.elapsed());
}
fn f3() {
    black_box(&A); //추가됨
    thread::spawn(|| loop {
        black_box(A.load(Relaxed));
    });

    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(A.load(Relaxed));
    }
    println!("{:?}", start.elapsed());
}

fn f4() {
    black_box(&A); //추가됨
    thread::spawn(|| loop {
        A.store(0, Relaxed);
    });

    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(A.load(Relaxed));
    }
    println!("{:?}", start.elapsed());
}
fn f5() {
    black_box(&A); //추가됨
    thread::spawn(|| loop {
        B[0].store(0, Relaxed);
        B[2].store(0, Relaxed);
    });

    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(B[1].load(Relaxed));
    }
    println!("{:?}", start.elapsed());
}
pub fn main() {
    black_box(&A); //추가됨
    thread::spawn(|| loop {
        C[0].0.store(0, Relaxed);
        C[2].0.store(0, Relaxed);
    });

    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(C[1].0.load(Relaxed));
    }
    println!("{:?}", start.elapsed());
}
