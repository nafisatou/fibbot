use std::env;

fn main() {
    // Read inputs from environment variables (GitHub Actions sets them)
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or("true".to_string());
    let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or("1000".to_string());

    println!("Enable Fibonacci Calculation: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    println!("Hello, world!");
}




