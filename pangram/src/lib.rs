use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .to_ascii_lowercase()
        .chars()
        .collect::<HashSet<_>>()
        .is_superset(&"abcdefghijklmnopqrstuvwyxz".chars().collect())
}
