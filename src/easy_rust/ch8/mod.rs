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
pub fn example() {
    ch8_7();
}
