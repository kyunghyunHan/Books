use rand::prelude::*;
use rand_distr::{Distribution, Normal};
use std::f64;

pub fn main() {
    // Parameters from the Python code
    let s: f64 = 100.0; // Initial stock price
    let k: f64 = 105.0; // Strike price
    let t: f64 = 1.0; // Time to maturity
    let r: f64 = 0.05; // Risk-free rate
    let sigma: f64 = 0.2; // Volatility
    let i: i32 = 100000; // Number of simulations

    // Set random seed for reproducibility
    let mut rng = StdRng::seed_from_u64(1000);

    // Generate standard normal random variables
    let normal = Normal::new(0.0, 1.0).unwrap();
    let z: Vec<f64> = (0..i).map(|_| normal.sample(&mut rng)).collect();

    // Calculate stock prices at maturity
    let st: Vec<f64> = z
        .iter()
        .map(|&z_val| s * f64::exp((r - sigma.powi(2) / 2.0) * t + sigma * f64::sqrt(t) * z_val))
        .collect();

    // Calculate payoff
    let nt: Vec<f64> = st.iter().map(|&st_val| f64::max(st_val - k, 0.0)).collect();

    // Calculate call option value
    let c0: f64 = f64::exp(-r * t) * nt.iter().sum::<f64>() / i as f64;

    println!("Value of the European call option: {:.3}", c0);
}
