pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .as_bytes()
        .windows(len)
        .filter_map(|f| std::str::from_utf8(f).ok().map(|s| s.to_string()))
        .collect()
}
