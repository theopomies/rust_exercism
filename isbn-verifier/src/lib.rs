/// Determines whether the supplied string is a valid ISBN number
/// I do not use enumerate() because I still need to get the last
/// Idx, which would imply |(acc, len), (idx, digit)|,
/// I still get the same result with less bloat
pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.chars()
        .filter(|c| c != &'-')
        .rev()
        .try_fold((0, 1), |(acc, len), digit| {
            let increment = match (digit, len) {
                ('X', 1) => 10,
                _ => digit.to_digit(10)?,
            };
            Some((acc + increment * len, len + 1))
        })
        .map(|(acc, len)| acc % 11 == 0 && len == 11)
        .unwrap_or(false)
}
