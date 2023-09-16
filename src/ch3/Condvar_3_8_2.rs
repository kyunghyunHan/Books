/*조건 변수 */

use std::sync::{Arc,Mutex,Condvar};
use std::thread;

//Condvar 타입의 변수가 조건 변수이며
//Mutex와 Condvar를 포함하는 튜플이 Arc에 포함되어 전달된다.
//대기 스레드용 함수정의,스레드 고유의 번호를 받는 Id변수 및 Mutex타입 변수와 Conver타입 변수의 튜플을 Arc로 감싼 값을 받는다.
fn child(id:u64,p:Arc<(Mutex<bool>,Condvar)>){ 
    let  &(ref lock,ref cvar)= &*p;

    //먼저 뮤텍스락을 수행한다.//
    //Arc타입 내부에 포함된 뮤텍스 변수와 조건 변수를 꺼낸다
    let mut started= lock.lock().unwrap();
    while !*started {//Mutex안의 공유변수가 flase인 동안 루프
       //wait으로 대기
       started = cvar.wait(started).unwrap();//알림이있을떄까지 대기한다
    }

    //다음과 같이 wait_while을 사용할수 있다.
    //cvar.wait_while(started|started|!*started).unwrap()
    println!("child {}",id);
}

fn parend(p:Arc<(Mutex<bool>,Condvar)>){//알림 스레드용 함수
    let &(ref lock,ref cvar)= &*p;

    //먼저 뮤텍스락을 수행한다.
    //락을 한뒤 공유 변수 값을 true로 설정하고 알림
    let mut started= lock.lock().unwrap();
    *started= true;//공유 변수 업데이트
    cvar.notify_all();//알림
    println!("parent");
}
fn main(){
    //뮤텍스와 조건 변수 작성
    let piar0 = Arc::new((Mutex::new(false),Condvar::new()));
    let pair1= piar0.clone();
    let pair2= piar0.clone();

    let c0= thread::spawn(move||{child(0, piar0)});
    let c1= thread::spawn(move||{child(1, pair1)});
    let p = thread::spawn(move||{parend(pair2)});

    c0.join().unwrap();
    c1.join().unwrap();
    p.join().unwrap();
}