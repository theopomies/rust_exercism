const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const WORD_LEN: usize = 5;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    convert_characters(plain)
        .enumerate()
        .map(|(idx, char)| match idx % WORD_LEN == 0 && idx != 0 {
            true => format!(" {char}"),
            false => char.to_string(),
        })
        .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    convert_characters(cipher).collect()
}

fn convert_characters(input: &str) -> impl Iterator<Item = char> + '_ {
    input
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|char| {
            let char = char.to_ascii_lowercase();
            let idx = match ALPHABET.find(char) {
                Some(idx) => idx,
                _ => return char,
            };
            let new_idx = ALPHABET.len() - idx - 1;
            ALPHABET.chars().nth(new_idx).unwrap()
        })
}
