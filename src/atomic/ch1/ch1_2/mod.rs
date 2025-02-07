use std::thread;

pub fn main() {
    //scoped thread
    //동시에 가변참조와 불변참조를가질수 없음
    let mut numbers = vec![1, 2, 3];
    //지역 변수를 입력받는 클로저와 범위과동일한 스레드
    thread::scope(|s| {
        // s.spawn(|| {
        //     println!("length: {}", numbers.len());
        // });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
        //numbers 에 새로운값을 넣으면 error
        // s.spawn(||{
        //     numbers.push(1);
        // });
    });
    println!("{numbers:?}");
}
