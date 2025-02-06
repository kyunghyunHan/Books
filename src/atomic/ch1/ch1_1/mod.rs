use std::thread;

pub fn main() {
    thread::spawn(f);
    thread::spawn(f);
    println!("Hello from the main thread");

    // let t1 = thread::spawn(f);
    // let t2 = thread::spawn(f);
    // println!("Hello from the main thread");

    // t1.join().unwrap();
    // t2.join().unwrap();
}

fn f() {
    println!("Hello from another thread!");
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
