use std::thread;

pub fn main() {
    thread::spawn(f);
    thread::spawn(f);
    println!("Hello from the main thread");

    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);
    println!("Hello from the main thread");

    t1.join().unwrap();
    t2.join().unwrap();//작업이 끝날떄 까지 기다리기

    let numbers = vec![1, 2, 3];
    //변수 numbers의 소유권이 새로 만들어진 스레드로 이동
    //move를 사용하지 않았다면 에러발생 소유권을 빌린 numbers보다 스레드가 더 오래 지속할수도 있기떄문
    thread::spawn(move || {
        for n in numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();

    // println!("{numbers:?}");

    let numbers = Vec::from_iter(0..=1000);
    //spawn함수는 'static라이프 타임을 갖는 타입을 입력받음
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });
    let average = t.join().unwrap();
    println!("average:{average}");
}

fn f() {
    println!("Hello from another thread!");
    let id = thread::current().id(); //스레드 식별자
    println!("This is my thread id: {id:?}");
}
