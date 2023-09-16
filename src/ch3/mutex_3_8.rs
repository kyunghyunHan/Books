/*mutex */

//뮤텍스는 MUTual Execution 의 약어이며 배타 실행 이라고도 불리는 동기처리 방법이다.
//크리티컬 섹션을 실행할수 있는 프로세스 수를 최대 1개로 제한하는 동기 처리다.
//베타적 실행을 위해 공유 변수로 사용할 플래그를 준비하고 해당 플래그가 true면 크리티컬 섹션을 실행하고 그렇지 않으면 실행하지 않는 처리를 생각할수 있다.



//동기 처리에 필요한 타입 import
//arc는 스레드 세이프한 참조 카운터 타입의 스마트 포인터를 구현한 타입,Mutex는 뮤텍스를 구현한 타입
use std::sync::{Arc,Mutex};
use std::thread;
fn some_func(lock:Arc<Mutex<u64>>){
    loop{
        //락을 하지 않으면 Mutex타입 안의 값은 참조 불가
        //lock함수를 호출해 락을 걸어 보호대상 데이터의 참조를 얻는다.
        //c에서는 보호대상 데이터는 락을 하지않아도 접근할수 있지만 레이스 컨디션이 될 가능서이 있다.
        //MutexGuard변수의 스코프를 벗어날떄 자동으로 락을 해제하는 구조가 구현되어 있다.
        let mut val = lock.lock().unwrap();
        *val +=1;
        println!("{}",*val);
    }
}
pub fn main(){
//Arc는 스레드 세이프한 참조 카운터 타입의 스마트 포인터
//뮤텍스용 변수를저장하는 스레드 세이프한 참조카운터 타입의 스마트 포인터를 생성한다.
//뮤텍스 변수는 이미 값을 저장하고 있으므로 초기값을 0 으로 설정한다.
let lock0 = Arc::new(Mutex::new(0));

//참조 카운터가 증가될 뿐이며 내용은 클론되지 않음
//Arc타입의 값은 클론해도 내부 데이터 복사는 하지않고 참조 카운터만 증가된다.
let lock1= lock0.clone();

//스레드 생성
//클로저 내 변수로 이동
//move지정자는 클로저안의 변수 캡처 방법을 지정한다.move가 지정되면 소유권이 이동하고,지정되지 않으면 참조가 전달된다.
let th0 = thread::spawn(move||{
    some_func(lock0);
});
//스레드 생성
//클로저 내 변수로 이동
let th1= thread::spawn(move||{
    some_func(lock1);
});
//약속
th0.join().unwrap();
th1.join().unwrap();


}