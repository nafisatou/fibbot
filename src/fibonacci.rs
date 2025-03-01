pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0
    }
    if n == 1 {
        return 1
    } else {
        return fibonacci(n - 1,) + fibonacci(n - 2,)
    }
}

// pub fn calculate_fibonacci_for_numbers(numbers: &[u32], max_threshold: u32) -> Vec<u32> {
//     let mut memo = HashMap::new();
//     numbers.iter().filter(|&&n| n <= max_threshold).map(|&n| fibonacci(n, &mut memo)).collect()
// }
