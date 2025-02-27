use std::collections::HashMap;

// Optimized Fibonacci function with memoization
pub fn fibonacci(n: u32, memo: &mut HashMap<u32, u32>) -> u32 {
    if n <= 1 {
        return n;
    }
    if let Some(&val) = memo.get(&n) {
        return val; // Return cached value if exists
    }
    
    let result = fibonacci(n - 1, memo) + fibonacci(n - 2, memo);
    memo.insert(n, result); // Cache the result
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let mut memo = HashMap::new();
        assert_eq!(fibonacci(0, &mut memo), 0);
        assert_eq!(fibonacci(1, &mut memo), 1);
        assert_eq!(fibonacci(2, &mut memo), 1);
        assert_eq!(fibonacci(3, &mut memo), 2);
        assert_eq!(fibonacci(5, &mut memo), 5);
        assert_eq!(fibonacci(10, &mut memo), 55);
        assert_eq!(fibonacci(20, &mut memo), 6765);
    }
}
