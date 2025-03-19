use core::num;

fn ch8_1() {
    let months = vec![
        "January",
        "February",
        "March",
        "Aprill",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let filtered_months = months
        .into_iter()
        .filter(|month| month.len() < 5)
        .filter(|month| month.contains("u"))
        .collect::<Vec<&str>>();

    println!("{:?}", filtered_months);
}
struct Company {
    name: String,
    ceo: Option<String>,
}
impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            name => Some(name.to_string()),
        };
        Self {
            name: name.to_string(),
            ceo: ceo,
        }
    }

    fn get_cet(&self) -> Option<String> {
        self.ceo.clone()
    }
}

fn ch8_2() {
    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Brendan McCracken"),
        Company::new("The Rad-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];
    let mut result_vec = vec![];
    company_vec.iter().for_each(|company| {
        result_vec.push(company.get_cet().ok_or_else(|| {
            let err_message = format!("No Ceo found for {}", company.name);

            err_message
        }));
    });

    for item in result_vec {
        println!("{:?}", item);
    }
}

fn ch8_3() {
    let new_vec = vec!["8", "9", "Ninetyniney"];
    let mut empty_vec = vec![];

    for index in 0..5 {
        empty_vec.push(
            new_vec
                .get(index)
                .and_then(|number| number.parse::<i32>().ok())
                .and_then(|number| f64::try_from(number).ok()),
        );
    }
    println!("{:?}", empty_vec);
}

fn ch8_4() {
    let mut big_vec = vec![6, 1000];
    big_vec.push(5);
    println!("{:?}", big_vec.iter().rev().any(|&number| number == 5));

    let num_vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    println!("{:?}", num_vec.iter().find(|&number| number % 3 == 0));
    println!("{:?}", num_vec.iter().find(|&number| number * 2 == 30));
    //위치를 나타냄
    println!("{:?}", num_vec.iter().position(|&number| number % 3 == 0));
}
//cycle을 사영하면 끊임없이 반복
fn ch8_5() {
    let even_odd = vec!["even", "odd"];
    let even_odd_vec = (0..6)
        .zip(even_odd.into_iter().cycle())
        .collect::<Vec<(i32, &str)>>();
    println!("{:?}", even_odd_vec);
}

fn ch8_6() {
    let ten_chars = ('a'..).take(10).collect::<Vec<char>>();
    let skip_then_ten_chars = ('a'..).skip(1300).take(10).collect::<Vec<char>>();

    println!("{:?}", ten_chars);
    println!("{:?}", skip_then_ten_chars);
}

fn ch8_7() {
    let some_numbers = vec![9, 6, 9, 10, 11];
    println!(
        "{}",
        some_numbers
            .iter()
            .fold(0, |total_fo_far, next_number| total_fo_far + next_number)
    );
}

fn ch8_8() {
    let a_string = "i don't have any dashes in me";

    println!(
        "{}",
        a_string
            .chars()
            .fold("-".to_string(), |mut string_so_far, next_char| {
                //문자열 - 로시작
                string_so_far.push(next_char);
                string_so_far.push('_');
                string_so_far
            })
    )
}
/*
by_ref :이데이터를 참조로변환
*/
fn ch8_9() {
    let mut number_vec = vec![7, 8, 9, 10].into_iter();
    let first_two = number_vec.by_ref().take(2).collect::<Vec<_>>();
    let second_two = number_vec.take(2).collect::<Vec<_>>();
}

fn ch8_10() {
    let num_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    for chunk in num_vec.chunks(3) {
        println!("{:?}", chunk);
    }

    println!();

    for window in num_vec.windows(3) {
        println!("{:?}", window);
    }
}
fn ch8_11() {
    let rules = "Rule number 1: No fighting. Rule number 2: Go to bed at 8pm. Rule number 3: Wake up at 6am.";
    let rule_locations = rules.match_indices("Rule").collect::<Vec<_>>();

    println!("{:?}", rule_locations);
}

fn ch8_12() {
    let just_numbers = vec![1, 5, 100];
    let mut number_iter = just_numbers.iter().peekable();

    for _ in 0..3 {
        println!("I love the number {}", number_iter.peek().unwrap());
        println!("I really love the number {}", number_iter.peek().unwrap());
        println!("{} is such a nuce number", number_iter.peek().unwrap());
        number_iter.next();
    }
}

fn ch8_13() {
    let locations = vec![
        ("Nevis", 25),
        ("Taber", 8428),
        ("Markeville", 45),
        ("Cardston", 3585),
    ];

    let mut location_iter = locations.iter().peekable();
    while location_iter.peek().is_some() {
        match location_iter.peek() {
            Some((name, number)) if *number < 100 => {
                println!("Found a hamlet:{name} with {number} people");
            }
            Some((name, number)) => println!("Found a town: {name} with {number} people"),
            None => break,
        }
        location_iter.next();
    }
}
pub fn example() {
    ch8_13();
}
