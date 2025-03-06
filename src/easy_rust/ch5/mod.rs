use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
struct City {
    name: String,
    population: HashMap<u32, u32>, //해당 연도와 해당 연도의 인구가표시
}
struct City2 {
    name: String,
    population: BTreeMap<u32, u32>, //해당 연도와 해당 연도의 인구가표시
}
fn ch5_1() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: HashMap::new(),
    };

    tallinn.population.insert(1322, 3_250);
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_619);

    for (year, population) in tallinn.population {
        //HashMap은 매번 두개의 항목을 반환

        println!(
            "In the year {} the city of {} had a population of {}",
            year, tallinn.name, population
        );
    }
}
fn ch5_2() {
    let mut tallinn = City2 {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(),
    };

    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_619);

    for (year, population) in tallinn.population {
        //HashMap은 매번 두개의 항목을 반환

        println!(
            "In the year {} the city of {} had a population of {}",
            year, tallinn.name, population
        );
    }
}

fn ch5_3() {
    let book_collection = vec!["L Allemagne Moderne", "Le prtit", "a", "a", "a"];
    let mut book_hashmap = HashMap::new();

    for book in book_collection {
        //return value는 변경가능한 참조
        let return_valye = book_hashmap.entry(book).or_insert(0);
        *return_valye += 1;
        // book_hashmap.entry(book).or_insert(true);
    }

    for (book, true_or_false) in book_hashmap {
        println!("{book} {true_or_false}");
    }
}
fn ch5_4() {
    let many_numbers = vec![
        94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81, 66,
        51, 58, 34, 59, 44, 19, 93, 28, 33, 18, 46, 61, 75, 14, 87, 84, 73, 71, 29, 94, 10, 35, 20,
        35, 80, 8, 43, 79, 25, 60, 26, 11, 37, 94, 32, 90, 51, 11, 20, 76, 16, 63, 95, 13, 50, 59,
        96, 95, 55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 86, 82, 58, 93, 68, 54, 80, 56, 22, 67, 82,
        58, 64, 80, 16, 61, 57, 14, 11,
    ];

    let mut number_hashset = HashSet::new();

    for number in many_numbers {
        number_hashset.insert(number);
    }

    let hashset_length = number_hashset.len();
    // .len()은 얼마나 많은 수가 있는지 알려 줍니다.

    println!(
        "There are {} unique numbers, so we are missing {}.",
        hashset_length,
        100 - hashset_length
    );

    // 우리가 놓친 숫자를 살펴봅시다.
    let mut missing_vec = vec![];
    for number in 0..100 {
        if number_hashset.get(&number).is_none() {
            // .get()이 None을 반환하는 경우
            missing_vec.push(number);
        }
    }

    print!("It does not contain: ");
    for number in missing_vec {
        print!("{number} ");
    }

    for entry in number_hashset {
        print!("{entry} ");
    }
}
fn ch5_5() {
    let many_numbers = vec![
        94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81, 66,
        51, 58, 34, 59, 44, 19, 93, 28, 33, 18, 46, 61, 75, 14, 87, 84, 73, 71, 29, 94, 10, 35, 20,
        35, 80, 8, 43, 79, 25, 60, 26, 11, 37, 94, 32, 90, 51, 11, 20, 76, 16, 63, 95, 13, 50, 59,
        96, 95, 55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 86, 82, 58, 93, 68, 54, 80, 56, 22, 67, 82,
        58, 64, 80, 16, 61, 57, 14, 11,
    ];

    let mut number_hashset = BTreeSet::new();

    for number in many_numbers {
        number_hashset.insert(number);
    }

    let hashset_length = number_hashset.len();
    // .len()은 얼마나 많은 수가 있는지 알려 줍니다.

    println!(
        "There are {} unique numbers, so we are missing {}.",
        hashset_length,
        100 - hashset_length
    );

    // 우리가 놓친 숫자를 살펴봅시다.
    let mut missing_vec = vec![];
    for number in 0..100 {
        if number_hashset.get(&number).is_none() {
            // .get()이 None을 반환하는 경우
            missing_vec.push(number);
        }
    }

    print!("It does not contain: ");
    for number in missing_vec {
        print!("{number} ");
    }
}

fn show_remainder(input: &BinaryHeap<i32>) -> Vec<i32> {
    // 이 함수는 BinaryHeap의 나머지를 보여 줍니다. 실제로 이터레이터는
    // 함수보다 확률니다. 이터레이터는 나중에 배울 것입니다.
    let mut remainder_vec = vec![];
    for number in input {
        remainder_vec.push(*number)
    }
    remainder_vec
}

fn ch5_6() {
    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30]; // 숫자는 순서대로 되어 있습니다.

    let mut my_heap = BinaryHeap::new();

    for number in many_numbers {
        my_heap.push(number);
    }

    while let Some(number) = my_heap.pop() {
        // .pop()은 힙에서 숫자를 꺼내 Some(숫자)를 반환하고,
        // 비어 있으면 None을 반환합니다.
        println!("Popped off {number}");
    }
}


fn ch5_7() {
   let mut jobs = BinaryHeap::new();

   // 하루 중의 할 일 추가
   jobs.push((100, "Write back to email from the CEO"));
   jobs.push((80, "Finish the report today"));
   jobs.push((5, "Watch some YouTube"));
   jobs.push((70, "Tell your team members thanks for always working hard"));
   jobs.push((30, "Plan who to hire next for the team"));

   while let Some(job) = jobs.pop() {
       println!("You need to: {}", job.1);
   }
}
use std::collections::VecDeque;

fn check_remaining(input: &VecDeque<(&str, bool)>) { // 각 항목은 (&str, bool)
   for item in input {
       if item.1 == false {
           println!("You must: {}", item.0);
       }
   }
}

fn done(input: &mut VecDeque<(&str, bool)>) {
   let mut task_done = input.pop_back().unwrap(); // 뒤쪽에서 꺼냅니다.
   task_done.1 = true; // '완료'를 의미하는 true로 변경합니다.
   input.push_front(task_done); // 앞쪽에 넣어 봅니다.
}

fn ch5_8() {
   let mut my_vecdeque = VecDeque::new();
   let things_to_do = vec!["send email to customer", "add new product to list", 
                         "phone Loki back"];
   
   for thing in things_to_do {
       my_vecdeque.push_front((thing, false));
   }
   
   done(&mut my_vecdeque);
   done(&mut my_vecdeque);
   
   check_remaining(&my_vecdeque);
   
   for task in my_vecdeque {
       print!("{task:?} ");
   }
}
pub fn example() {
    ch5_8();
}
