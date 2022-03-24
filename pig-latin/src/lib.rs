const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];

const SUFFIX: &str = "ay";

pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn translate_word(input: &str) -> String {
    let (consonants, vowels) = input.split_at(consonants(input));

    format!("{vowels}{consonants}{SUFFIX}")
}

fn consonants(word: &str) -> usize {
    if word.is_empty()
        || word.starts_with(VOWELS)
        || word.starts_with("yt")
        || word.starts_with("xr")
    {
        0
    } else if let Some(stripped) = word.strip_prefix("qu") {
        2 + consonants(stripped)
    } else if word.chars().nth(1) == Some('y') {
        1
    } else {
        1 + consonants(&word[1..])
    }
}
