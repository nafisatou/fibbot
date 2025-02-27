use std::env;

fn hello_world() {
    println!("Hello, world! FibBot is running...");
}

fn extract_numbers(text: &str) -> Vec<u32> {
    text.split_whitespace()
        .filter_map(|word| word.parse::<u32>().ok())
        .filter(|&num| (5..=15).contains(&num)) // Limit to numbers between 5 and 15
        .collect()
}

fn fibonacci(n: u32) -> u32 {  // Keep u32 for Fibonacci
    (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0

    
}

fn main() {
    //Day 1: Setup & Hello World 
    hello_world();
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or("true".to_string()) == "true";
    let max_threshold: u32 = env::var("INPUT_MAX_THRESHOLD").unwrap_or("100".to_string()).parse().unwrap_or(100);
    println!("Enable Fibonacci: {}, Max Threshold: {}", enable_fib, max_threshold);
    // End of Day 1 

    // Day 2: Extract Numbers & Implement Fibonacci 
    let numbers = extract_numbers("PR fixes issues 5 and 10, improves 7 cases.");
    println!("Extracted Numbers: {:?}", numbers);
    //  End of Day 2

    // Day 3: Validate Inputs & Apply Threshold 
    if enable_fib {
        for &num in &numbers {
            if num <= max_threshold {
                println!("Fib({}) = {}", num, fibonacci(num));
            }
        }
    } else {
        println!("Fibonacci calculation is disabled.");
    }
    //  End of Day 3 


}

