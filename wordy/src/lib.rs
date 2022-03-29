pub fn answer(command: &str) -> Option<i32> {
    let input: Vec<&str> = command
        .strip_prefix("What is ")?
        .strip_suffix('?')?
        .split_ascii_whitespace()
        .collect();
    let first_value = input.get(0)?.parse::<i32>().ok()?;
    parse_operation(first_value, &input[1..])
}

fn parse_operation(lhs: i32, input: &[&str]) -> Option<i32> {
    match input {
        [] => Some(lhs),
        ["plus", rhs, ref rest @ ..] => {
            let rhs = rhs.parse::<i32>().ok()?;
            parse_operation(lhs + rhs, rest)
        }
        ["minus", rhs, ref rest @ ..] => {
            let rhs = rhs.parse::<i32>().ok()?;
            parse_operation(lhs - rhs, rest)
        }
        ["multiplied", "by", rhs, ref rest @ ..] => {
            let rhs = rhs.parse::<i32>().ok()?;
            parse_operation(lhs * rhs, rest)
        }
        ["divided", "by", rhs, ref rest @ ..] => {
            let rhs = rhs.parse::<i32>().ok()?;
            parse_operation(lhs / rhs, rest)
        }
        ["raised", "to", "the", rhs, "power", ref rest @ ..] => {
            let rhs = rhs
                .trim_end_matches("st")
                .trim_end_matches("nd")
                .trim_end_matches("rd")
                .trim_end_matches("th")
                .parse::<u32>()
                .ok()?;
            parse_operation(lhs.pow(rhs), rest)
        }
        _ => None,
    }
}
