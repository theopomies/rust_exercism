use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .to_ascii_lowercase()
        .chars()
        .fold(HashSet::new(), |mut set, c| {
            set.insert(c);
            set
        })
        .is_superset(&"abcdefghijklmnopqrstuvwyxz".chars().collect())
}
