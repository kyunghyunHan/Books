pub fn main() {
    let a = 3;
    let mut b = 3;
    f(&a, &mut b);
}

fn f(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;
    if before != after {
        x();
    }
}
fn x() {
    println!("절대 실행되지않음");
}
