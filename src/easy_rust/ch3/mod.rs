use std::cmp::PartialOrd;
use std::fmt::Display;

pub fn ch3_1_2() {
    let mut vec: Vec<u8> = [1, 2, 3].into();
    let mut num_vec = Vec::with_capacity(8);
    //동적이라 가능
    for i in 0..100 {
        num_vec.push(i);
    }

    println!("{}", num_vec.capacity());
    println!("{:?}", num_vec)
}

fn ch3_2() {
    let childred = 5;
    let married = true;

    match (childred, married) {
        (c, m) if m == false => {
            println!("Not marrid with {} childred", childred)
        }

        _ => println!("Married?"),
    }
}

fn ch3_3() {
    let mut counter = 0;
    let mut counter2 = 0;

    println!("Now entering the first loop");

    'first_loop: loop {
        //첫번쨰 루프에 이름을 지정
        counter += 1;
        println!("The counter is now:{}", counter);
        if counter > 5 {
            //이 루프 안에서 두번쨰 루프를 실행
            println!("Now entering the second loop");

            'second_loop: loop {
                //이제 secondloop
                println!("The scond counter is now :{}", counter2);
                counter2 += 1;
                if counter2 == 10000 {
                    break 'first_loop;
                }
            }
        }
    }
}

struct Person {
    name: String,
    real_name: String,
}
fn ch3_4() {
    let papa_doc = Person {
        name: "papa DOC".to_string(),
        real_name: "Clarence".to_string(),
    };

    let Person {
        name: a,
        real_name: b,
    } = papa_doc;

    println!("{}", a);
    println!("{}", b);
}
//도트연산자는 *가 필요하지 않음
// fn compare_and_disply<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
//     println!("{statement} , {} , {} ,{}", num_1, num_2, num_1 < num_2)
// }
//제네릭 타입이 많다면 where
fn compare_and_disply<T, U>(statement: T, num_1: U, num_2: U)
where
    T: Display,
    U: Display + PartialOrd,
{
    println!("{statement} , {} , {} ,{}", num_1, num_2, num_1 < num_2)
}
fn ch3_5() {
    compare_and_disply("Listem", 9, 8);
}

fn ch3_6() {
    let weather_vec = vec![
        vec!["berlin", "cloudy", "5", "-7", "78"],
        vec!["Athens", "sunny", "not humid", "20", "10", "50"],
    ];

    for mut city in weather_vec {
        println!("For the city of {}:", city[0]);

        while let Some(information) = city.pop() {
            //더이상 꺼낼수 없을떄까지 진행

            if let Ok(number) = information.parse::<i32>() {
                //information 이라는 변수를 i32로 변경
                //반환된 결과가 Ok이면 출력
                println!("The number is :{number}");
            }

            //오류가 발생하면 아무것도 하지않음
        }
    }
}
pub fn example() {
    ch3_6();
}
