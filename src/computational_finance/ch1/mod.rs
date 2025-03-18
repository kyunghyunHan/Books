use std::f64::consts::E;

// Simple interest calculation function
fn simple_interest(principal: f64, rate: f64, time: f64) -> f64 {
    // Simple interest: interest is only earned on principal
    principal * (1.0 + rate * time)
}

// Continuous compounding calculation function
fn continuous_compound(principal: f64, rate: f64, time: f64) -> f64 {
    // Continuous compounding: Mt = M(t0) * e^(rt)
    principal * E.powf(rate * time)
}

// Regular compounding calculation function
fn compound_interest(principal: f64, rate: f64, time: f64, n: f64) -> f64 {
    // Regular compounding: interest is earned on principal+interest, n is number of times compounding occurs per year
    principal * (1.0 + rate / n).powf(n * time)
}

pub fn example() {
    // Initial investment
    let principal: f64 = 1000000.0;  // $1 million
    let interest_rate: f64 = 0.05;   // 5% annual interest rate
    let years: i32 = 10;             // 10-year investment period

    // Calculate values for each year
    let mut simple_values = Vec::new();
    let mut continuous_values = Vec::new();
    let mut annual_compound_values = Vec::new();
    let mut monthly_compound_values = Vec::new();

    for t in 0..=years {
        let time = t as f64;
        simple_values.push(simple_interest(principal, interest_rate, time));
        continuous_values.push(continuous_compound(principal, interest_rate, time));
        annual_compound_values.push(compound_interest(principal, interest_rate, time, 1.0));
        monthly_compound_values.push(compound_interest(principal, interest_rate, time, 12.0));
    }

    // Output results
    println!("Initial investment: ${:.2}", principal);
    println!("Interest rate: {}% annually", interest_rate * 100.0);
    println!("Investment period: {} years", years);
    println!("\n[Final Investment Values]");
    println!("Simple interest: ${:.2}", simple_values.last().unwrap());
    println!("Annual compounding: ${:.2}", annual_compound_values.last().unwrap());
    println!("Monthly compounding: ${:.2}", monthly_compound_values.last().unwrap());
    println!("Continuous compounding: ${:.2}", continuous_values.last().unwrap());
    

}