pub fn extract_numbers_from_pr(pr_content: &str) -> Vec<u32> {
    pr_content
        .split_whitespace()
        .filter_map(|word| word.parse::<u32>().ok()) // Parse words into numbers
        .collect()
}
