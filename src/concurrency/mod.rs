use std::thread;
fn f(){
    println!("Hello from another thread!");
    let id  = thread::current().id();
    println!("This is my thread: {id:?}");
}

pub fn main(){
   thread::spawn(f);
   thread::spawn(f);
   println!("Hello from the main thread");
}