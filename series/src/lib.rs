pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        return vec![]
    } else if len == 0 {
        vec!["".to_string(); digits.len() + 1]
    } else {
        let chars: Vec<char> = digits.chars().collect();
        chars
            .windows(len)
            .map(|slice| slice.into_iter().collect::<String>())
            .collect()
    }
}
