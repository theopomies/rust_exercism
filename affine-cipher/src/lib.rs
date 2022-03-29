/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const WORDS_LEN: usize = 5;

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, ALPHABET.len() as i32) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    Ok(plaintext
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|char| {
            let char = char.to_ascii_lowercase();
            let idx = match ALPHABET.find(char) {
                Some(idx) => idx as i32,
                _ => return char,
            };
            let new_idx = (a * idx + b) as usize % ALPHABET.len();
            ALPHABET.chars().nth(new_idx).unwrap()
        })
        .enumerate()
        .map(|(idx, char)| match idx % WORDS_LEN == 0 && idx != 0 {
            true => format!(" {char}"),
            false => char.to_string(),
        })
        .collect())
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, ALPHABET.len() as i32) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let mmi = find_mmi(a);
    Ok(ciphertext
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|char| {
            let idx = match ALPHABET.find(char) {
                Some(idx) => idx as i32,
                _ => return char,
            };
            let new_idx = (mmi * (idx - b)).rem_euclid(ALPHABET.len() as i32);
            ALPHABET.chars().nth(new_idx as usize).unwrap()
        })
        .collect())
}

/// Computes gcd, only use with positive numbers, can't use unsigned
/// because I follow the original exercise
fn gcd(lhs: i32, rhs: i32) -> i32 {
    match lhs {
        0 => rhs,
        _ => gcd(rhs % lhs, lhs),
    }
}

/// Naive implementation to find the MMI
fn find_mmi(a: i32) -> i32 {
    (1..ALPHABET.len() as i32)
        .find(|n| (a * n) % ALPHABET.len() as i32 == 1)
        .unwrap()
}
