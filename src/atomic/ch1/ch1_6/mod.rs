use std::cell::Cell;

fn f(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        x();
    }
}

fn x() {
    println!("{}", "execution");
}
pub fn main() {
    let a = Cell::new(10);
    let b = Cell::new(10);
    
    f(&a, &a);
}
