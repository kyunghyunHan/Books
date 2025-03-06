fn r#return() -> u8 {
    8
}

pub fn example() {
    let my_number = r#return();
    println!("{}", my_number);

    let number = 0;
    let number_ref = &number;
    println!("{:p}", number_ref);

    let number = 555;

    println!(
        "Binary:{:b} hexadecimal:{:x}, octal:{:o}",
        number, number, number
    );

    println!(
        "{city1} is in {country} and{city2} is also in{country} but {city3} is npt in {country}",
        city1 = "Seoul",
        city2 = "Busan",
        city3 = "Tokyo",
        country = "Korea"
    );

    let title = "Today's News";
    println!("{:-^30}", title);
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar);

    let a = "SEOUL";
    let b = "TOKYO";

    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);

    let country = String::from("Austria");
    print_country(&country);
    print_country(&country);
    let mut country2 = String::from("Austria");

    add_hungry(&mut country2);

    let mut country2 = 1;
}
fn print_country(country_name: &String) {
    println!("{}", country_name)
}
fn add_hungry2(country2: &mut i32) {
    *country2 += 1;
}
fn add_hungry(country2: &mut String) {
    country2.push_str("--Hungry");
}
