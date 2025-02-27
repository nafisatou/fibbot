mod numerc;      // Extracts numbers from PR content
mod fibonacci;   // Calculates Fibonacci numbers
mod github;      // Interacts with the GitHub API

use numerc::extract_numbers_from_pr;
use fibonacci::{fibonacci, calculate_fibonacci_for_numbers};
use github::post_comment_to_pr;

use std::env;

#[tokio::main]  // This attribute makes the `main` function async
async fn main() {
    // **Day 1: Setup and Action Review**
    println!("Hello, world! FibBot Action Initialized.");

    // **Day 2: Minimal Action Implementation - Hello World**
    // Confirm that the action is running by printing out basic info
    println!("FibBot Action Setup Complete.");

    // **Day 3: Parameter Handling & Input Parsing**
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

    // **Day 4: Core Logic - Extracting Numbers and Calculating Fibonacci**
    // Example PR content for testing
    let pr_content = "Here are some numbers: 3 5 8";  // Simulate PR content with numbers
    let numbers = extract_numbers_from_pr(pr_content);
    println!("Extracted numbers: {:?}", numbers);

    // Calculate Fibonacci numbers for the extracted numbers
    let fibonacci_results = calculate_fibonacci_for_numbers(&numbers, max_threshold);
    println!("Fibonacci results: {:?}", fibonacci_results);

    // **Day 5: GitHub API Interaction - Post Comment**
    // Removed comment posting logic due to the error encountered

    // **GitHub API credentials**
    let token = match env::var("GITHUB_TOKEN") {
        Ok(t) => t,
        Err(_) => {
            eprintln!("GitHub token is not set. Please export it as an environment variable.");
            return;
        }
    };

    // Proceed without posting the comment
    println!("GitHub token: [successfully]");  // Do not print the token for security reasons
}
