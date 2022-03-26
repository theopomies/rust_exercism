/// Compute the Scrabble score for a word.
/// Note: would love to use a const HashMap :(
pub fn score(word: &str) -> u64 {
    word.chars()
        .filter(char::is_ascii_alphabetic)
        .map(get_letter_score)
        .sum()
}

fn get_letter_score(letter: char) -> u64 {
    match letter.to_ascii_uppercase() {
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
        'D' | 'G' => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' => 8,
        'Q' | 'Z' => 10,
        _ => 0,
    }
}
