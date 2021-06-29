use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

fn reducer<'a>(
    mut set: HashSet<&'a str>,
    word: &str,
    possible_anagram: &'a str,
) -> HashSet<&'a str> {
    let lowercase_possible_anagram = possible_anagram.to_lowercase();

    if word == lowercase_possible_anagram {
        return set;
    }

    let mut word: Vec<_> = word.graphemes(true).collect();
    word.sort_unstable();

    let mut lowercase_possible_anagram: Vec<_> =
        lowercase_possible_anagram.graphemes(true).collect();
    lowercase_possible_anagram.sort_unstable();

    if word == lowercase_possible_anagram {
        set.insert(possible_anagram);
    }
    set
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    
    possible_anagrams
        .iter()
        .fold(HashSet::new(), |set, possible_anagram| {
            reducer(set, &word, possible_anagram)
        })
}
