pub fn abbreviate(name: &str) -> String {
    name.split(|c: char| c.is_whitespace() || "-_".contains(c))
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .collect::<String>()
        .to_uppercase()
}
