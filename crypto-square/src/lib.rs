pub fn encrypt(input: &str) -> String {
    let filtered: Vec<_> = input
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let c = (filtered.len() as f64).sqrt().ceil() as usize;

    (0..c)
        .map(|i| {
            filtered
                .chunks(c)
                .filter_map(|chunk| chunk.get(i).or(Some(&' ')))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
