#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

const ROWS_PER_DIGIT: usize = 4;
const COLS_PER_DIGIT: usize = 3;

const DIGITS: [[&str; ROWS_PER_DIGIT]; 10] = [
    [" _ ", "| |", "|_|", "   "],
    ["   ", "  |", "  |", "   "],
    [" _ ", " _|", "|_ ", "   "],
    [" _ ", " _|", " _|", "   "],
    ["   ", "|_|", "  |", "   "],
    [" _ ", "|_ ", " _|", "   "],
    [" _ ", "|_ ", "|_|", "   "],
    [" _ ", "  |", "  |", "   "],
    [" _ ", "|_|", "|_|", "   "],
    [" _ ", "|_|", " _|", "   "],
];

pub fn convert(input: &str) -> Result<String, Error> {
    Ok(input
        .lines()
        .collect::<Vec<_>>()
        .chunks(ROWS_PER_DIGIT)
        .map(|rows| {
            if rows.len() != ROWS_PER_DIGIT {
                return Err(Error::InvalidRowCount(rows.len()));
            }
            if let Some(row) = rows.iter().find(|row| row.len() % COLS_PER_DIGIT != 0) {
                return Err(Error::InvalidColumnCount(row.len()));
            }
            let mut digits = String::with_capacity(rows[0].len() / COLS_PER_DIGIT);
            for col in (0..rows[0].len()).step_by(COLS_PER_DIGIT) {
                let mut digit_slices = [""; ROWS_PER_DIGIT];
                for (idx, digit_slice) in rows
                    .iter()
                    .map(|row| &row[col..col + COLS_PER_DIGIT])
                    .enumerate()
                {
                    digit_slices[idx] = digit_slice;
                }
                digits.push_str(&parse_digit(&digit_slices));
            }
            Ok(digits)
        })
        .collect::<Result<Vec<String>, _>>()?
        .join(","))
}

fn parse_digit(digit: &[&str]) -> String {
    DIGITS
        .into_iter()
        .enumerate()
        .find(|(_, pattern)| pattern == digit)
        .map(|(value, _)| value.to_string())
        .unwrap_or_else(|| "?".into())
}
