mod fibonacci;   // Calculates Fibonacci numbers
mod github;      // Interacts with the GitHub API
mod extract_numbers;
mod get_pull;
use get_pull::get_pr;
use fibonacci::fibonacci;
use github::post_comment;

use std::env;

#[tokio::main]  // This attribute makes the `main` function async
async fn main() {
    // Day 1: Setup and Action Review
    println!("Hello, world! FibBot Action Initialized.");

    // Day 2: Minimal Action Implementation - Hello World
    // Confirm that the action is running by printing out basic info
    println!("FibBot Action Setup Complete.");

    // Day 3: Parameter Handling & Input Parsing
    // Get the inputs from the GitHub workflow (via environment variables)
    let enable_fib = match env::var("ENABLE_FIB") {
        Ok(val) => val == "true",  // Convert to boolean
        Err(_) => false,  // Default to false if not set
    };

    let max_threshold: u32 = match env::var("MAX_THRESHOLD") {
        Ok(val) => val.parse().unwrap_or(100),  // Default to 100 if parsing fails
        Err(_) => 100,  // Default to 100 if not set
    };

    println!("Enable Fibonacci Calculation: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    if !enable_fib {
        println!("Fibonacci calculation is disabled.");
        return;
    }

    // Day 4: Core Logic - Extracting Numbers and Calculating Fibonacci
    // Example PR content for testing
    let pr_number: u64 = env::var("PR_NUMBER")
        .expect("PR_NUMBER not set")
        .parse::<u64>()
        .expect("Invalid PR_NUMBER");  // Simulate PR content with numbers

    let pr_numbers = get_pr(pr_number).await;
   
    println!("Extracted numbers: {:?}", pr_numbers);

    // Calculate Fibonacci numbers for the extracted numbers
    let mut response =
        String::from("#### Fibonacci output of each number in the pull_request is:\n");
    for &num in &pr_numbers {
        let fib = fibonacci(num);
        response.push_str(&format!("- Fibonacci({}) = {}\n", num, fib));
    }
        if let Err(e) = post_comment(&response).await {
            eprintln!("Error posting comment: {}", e);
        }
}
