use std::sync::{Condvar,Mutex};
use std::sync::Arc;

use std::sync::atomic::{AtomicUsize,Ordering};
pub struct Semaphore {
    mutex:Mutex<isize>,
    cond:Condvar,
    max:isize,
}

impl Semaphore {
    pub fn new(max:isize)->Self {
        Semaphore{
            mutex:Mutex::new(0),
            cond:Condvar::new(),
            max
        }
   
    }      
    
    pub fn wait(&self) {
        //카운터가 최댓값 이상이면 대기
        let mut cnt = self.mutex.lock().unwrap();
        while *cnt >= self.max {
            cnt = self.cond.wait(cnt).unwrap();
        }
        *cnt +=1;
    }

    pub fn post(&self){
        //카운터 감소
        let mut cnt = self.mutex.lock().unwrap();
        *cnt -=1;

        if*cnt <= self.max {
            self.cond.notify_one();
        }
    }

}

const NUM_LOOP:usize= 100000;
const NUM_THREADS:usize= 8;
const SUM_NUM:isize = 4;

static  mut CNT:AtomicUsize= AtomicUsize::new(0);
//테스트코드
pub fn main(){
    let mut v= Vec::new();
    let sem= Arc::new(Semaphore::new(SUM_NUM));
    for i in 0..NUM_THREADS{
        let s= sem.clone();
        let t= std::thread::spawn(move||{
            for _ in 0..NUM_LOOP{
                s.wait();

                unsafe{
                    CNT.fetch_add(1, Ordering::SeqCst)
                };
                let n= unsafe {
                    CNT.load(Ordering::SeqCst)
                };
                println!("semaphore:i{},CNT ={}",i,n);
                assert!((n as isize )<=SUM_NUM);
                unsafe {CNT.fetch_sub(1, Ordering::SeqCst)};
                s.post();
            }
        });
        v.push(t);
    }
        for t in v {
            t.join().unwrap();
        }
    
    

}