pub fn example() {
    let doesnt_print= ();
    println!("This will not print: {:?}",doesnt_print);

    let a = "aa".repeat(3);

    println!("{a}");
}