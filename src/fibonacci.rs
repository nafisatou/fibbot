use std::collections::HashMap;

pub fn fibonacci(n: u32, memo: &mut HashMap<u32, u32>) -> u32 {
    if let Some(&val) = memo.get(&n) {
        return val;
    }
    let result = match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1, memo) + fibonacci(n - 2, memo),
    };
    memo.insert(n, result);
    result
}

pub fn calculate_fibonacci_for_numbers(numbers: &[u32], max_threshold: u32) -> Vec<u32> {
    let mut memo = HashMap::new();
    numbers.iter().filter(|&&n| n <= max_threshold).map(|&n| fibonacci(n, &mut memo)).collect()
}
