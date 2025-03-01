pub fn extract_numbers(text: &str) -> Vec<u32> {
    let mut numbers = Vec::new();
    let mut current_num = String::new();

    for ch in text.chars() {
        if ch.is_digit(10) {
            current_num.push(ch);
        } else if !current_num.is_empty() {
            if let Ok(num) = current_num.parse::<u32>() {
                if (10..=20).contains(&num) {
                    numbers.push(num);
                }
            }
            current_num.clear();
        }
    }

    // Push the last number if any
    if !current_num.is_empty() {
        if let Ok(num) = current_num.parse::<u32>() {
            if (..=15).contains(&num) {
                numbers.push(num);
            }
        }
    }

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numbers() {
        assert_eq!(extract_numbers("Numbers: 3, 5, 10, 20, 15."), vec![5, 10, 15]);
        // assert_eq!(extract_numbers("No numbers here!"), vec![]);
        assert_eq!(extract_numbers("7  and 9 ."), vec![7, 9]);
        assert_eq!(extract_numbers("5 10 15"), vec![5, 10, 15]);
    }
}
