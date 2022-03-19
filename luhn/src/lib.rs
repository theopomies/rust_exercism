/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    if code.len() < 2 {
        return false;
    }
    let parity = code.len() % 2;

    code.chars()
        .enumerate()
        .map(|(idx, c)| {
            let c = c.to_digit(10);
            if idx % 2 == parity {
                c.map(|c| if c * 2 > 9 { c * 2 - 9 } else { c * 2 })
            } else {
                c
            }
        })
        .collect::<Option<Vec<u32>>>()
        .map(|v| v.iter().sum())
        .unwrap_or(1)
        % 10
        == 0
}
