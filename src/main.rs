
// iterative vs functional

// Calculate Harmonic series:
// 1 + (1/2)^2 + (1/3)^3 + (1/4)^4 + ...
fn harmonic(n: i32) -> f64 {
    let mut sum: f64 = 0.0;
    for i in 1..n {
        sum += 1.0 / f64::from(i);
    }

    return sum;
}

fn harmonic_functional(n: i32) -> f64 {
    (1..n).map(f64::from).map(|i| 1.0 / i).sum()
}

// Error types, traits, polymorphism

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

static ERR: bool = true;

fn polymorphism() -> Result<(), MyError> {
    let vec1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let vec2: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    println!("Sum of vec1: {}", vec1.iter().sum::<i32>());
    println!("Sum of vec2: {}", vec2.iter().sum::<f64>());

    if ERR {
        return Err(MyError::new("Fuck..."));
    }

    Ok(())
}

extern crate simple_error;
use simple_error::SimpleError;

static CRATE_ERR: bool = true;

fn polymorphism_with_crate() -> Result<(), SimpleError> {
    let vec1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let vec2: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    println!("Sum of vec1: {}", vec1.iter().sum::<i32>());
    println!("Sum of vec2: {}", vec2.iter().sum::<f64>());

    if CRATE_ERR {
        return Err(SimpleError::new("Fuck... At least this time the errors were cleaner!"));
    }

    Ok(())
}

fn main() {
    println!("Harmonic with n = 10 is {}", harmonic(10));
    println!(
        "Harmonic (calculated functionally) with 10 is {}",
        harmonic_functional(10)
    );

    polymorphism().unwrap_or_else(|e| eprintln!("Yikes... Error message: {}", e));
    polymorphism_with_crate().unwrap_or_else(|e| eprintln!("Yikes... Error message: {}", e));
}
