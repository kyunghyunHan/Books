use ndarray::Array1;
use rand::prelude::*;
use rand::thread_rng;
use rand_distr::StandardNormal;

use std::f64;

pub fn main() {
    //유러피안 콜 옵션의 몬테카를로 가격 결정

    let s0: f64 = 100.0; // 초기 주가 지수
    let k: f64 = 105.0; //행사가
    let t: f64 = 1.0; //만기 까지 남은 시간(연)
    let r: f64 = 0.05; // 무위험 이자율
    let sigma: f64 = 0.2; // 변동성
    let i: i32 = 100000; // 시뮬레이션 횟수

    //가격 결정 알고리즘
    let mut rng = thread_rng();
    let normal = StandardNormal;

    let z: Array1<f64> = Array1::from_iter((0..i).map(|_| normal.sample(&mut rng)));
    // 만기 시 주가지수
    let st = z.map(|&zi| s0 * ((r - 0.5 * sigma.powi(2)) * t + sigma * f64::sqrt(t) * zi).exp());

    let ht = st.map(|&s| f64::max(s - k, 0.0));
    let c0 = (-r * t).exp() * ht.mean().unwrap();
    println!("Value of the European call option {:.3}", c0);
}
