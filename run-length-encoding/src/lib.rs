pub fn encode(source: &str) -> String {
    let (count, last_char, mut encoded) = source.chars().fold(
        (0, None, String::new()),
        |(count, last_char, mut encoded), char| {
            match (count, last_char) {
                (count, Some(c)) if c == char => return (count + 1, last_char, encoded),
                (_, None) => return (1, Some(char), encoded),
                (count, _) if count > 1 => encoded.push_str(count.to_string().as_str()),
                _ => (),
            }
            encoded.push(last_char.unwrap());
            (1, Some(char), encoded)
        },
    );
    match (count, last_char) {
        (1, Some(c)) => encoded.push(c),
        (count, Some(c)) => {
            encoded.push_str(count.to_string().as_str());
            encoded.push(c);
        }
        _ => (),
    }
    encoded
}

pub fn decode(source: &str) -> String {
    use std::iter::repeat;
    source
        .chars()
        .fold(
            (0, "".to_string()),
            |(count, mut decoded), char| match char.to_digit(10) {
                Some(digit) => (count * 10 + digit, decoded),
                None => {
                    let count = if count > 0 { count as usize } else { 1 };
                    decoded.push_str(repeat(char).take(count).collect::<String>().as_str());
                    (0, decoded)
                }
            },
        )
        .1
}
