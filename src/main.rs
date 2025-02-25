use std::env;

fn main() {
    // Read inputs from environment variables
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "true".to_string());
    let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or_else(|_| "100".to_string());

    // Validate and parse enable_fib (only allow "true" or "false")
    let enable_fib = match enable_fib.as_str() {
        "true" => true,
        "false" => false,
        _ => {
            eprintln!("Invalid enable_fib value. Using default: true");
            true
        }
    };

    // Validate max_threshold (must be a positive number)
    let max_threshold: u32 = match max_threshold.parse() {
        Ok(num) if num > 0 => num,
        _ => {
            eprintln!("Invalid max_threshold value. Using default: 100");
            100
        }
    };

    // Debug output
    println!("Enable Fibonacci: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);
}
