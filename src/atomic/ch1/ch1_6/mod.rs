use std::cell::Cell;
use std::marker::PhantomData;
struct X {
    handle: i32,
    _not_sync: PhantomData<Cell<()>>,
}
pub fn main() {}
