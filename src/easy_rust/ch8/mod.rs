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

fn 
pub fn example() {
    ch8_2();
}
