pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".into(); digits.len() + 1];
    }
    digits
        .chars()
        .collect::<Vec<_>>()
        .windows(len)
        .map(|chars| chars.iter().collect::<String>())
        .collect()
}
