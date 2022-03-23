/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.chars()
        .filter(|&c| c != '-')
        .rev()
        .enumerate()
        .map(|pos_digit| match pos_digit {
            (0, 'X') => Some((1, 10)),
            (pos, digit) => digit.to_digit(10).map(|digit| (pos as u32 + 1, digit)),
        })
        .try_fold((0, 0), |(acc, _), pos_digit| {
            pos_digit.map(|(pos, digit)| (acc + pos * digit, pos))
        })
        .map(|(acc, len)| acc % 11 == 0 && len == 10)
        .unwrap_or(false)
}
