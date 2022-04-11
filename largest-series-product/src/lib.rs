#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    match span {
        0 => Ok(1),
        _ => string_digits
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(|d| d as u64)
                    .ok_or(Error::InvalidDigit(c))
            })
            .collect::<Result<Vec<_>, _>>()?
            .windows(span)
            .map(|window| window.iter().product())
            .max()
            .ok_or(Error::SpanTooLong),
    }
}
