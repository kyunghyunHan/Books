use ndarray::{Array, Array1};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
fn ch3_1(){
    let n: Array1<usize> = Array::random(10000, Uniform::new(0, 10));
    let counts = bincount(&n, 2);
    println!("{:?}",counts);


}
fn bincount(arr: &Array1<usize>, minlength: usize) -> Array1<usize> {
    let max_val = arr.iter().max().copied().unwrap_or(0);
    let length = std::cmp::max(max_val + 1, minlength);

    let mut counts: Array1<usize> = Array::zeros(length);

    for &val in arr.iter() {
        counts[val] += 1;
    }

    counts
}

pub fn example() {
    ch3_1();
}