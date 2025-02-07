use std::cell::Cell;
/*
cell은 단순히 T를 감싸고 있는 타입으로공유 레퍼런스로 값을 변경할수 있는 타입

*/ 
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
