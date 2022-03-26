const UNITS: &[&str] = &[
    "",
    " thousand",
    " million",
    " billion",
    " trillion",
    " quadrillion",
    " quintillion",
];

pub fn encode(n: u64) -> String {
    match n {
        0 => "zero".into(),
        _ => {
            UNITS
                .iter()
                .fold(("".to_string(), n), |(encoded, value), unit| {
                    let current_block = value % 1000;
                    let rest = value / 1000;
                    let mut translated = hundreds(current_block);
                    match translated.is_empty() {
                        true => (encoded, rest),
                        false => {
                            translated.push_str(unit);
                            if !encoded.is_empty() {
                                translated.push(' ');
                                translated.push_str(encoded.as_str());
                            }
                            (translated, rest)
                        }
                    }
                })
                .0
        }
    }
}

fn hundreds(n: u64) -> String {
    let hundred = unit(n / 100);
    let rest = tens(n % 100);
    match (hundred.is_empty(), rest.is_empty()) {
        (true, true) => "".into(),
        (false, false) => format!("{hundred} hundred {rest}"),
        (false, true) => format!("{hundred} hundred"),
        (true, false) => rest,
    }
}

fn tens(n: u64) -> String {
    let mut base: String = match (n / 10, n % 10) {
        (0, u) => return unit(u).into(),
        (1, u) => {
            return match u {
                0 => "ten".into(),
                1 => "eleven".into(),
                2 => "twelve".into(),
                3 => "thirteen".into(),
                4 => "fourteen".into(),
                5 => "fifteen".into(),
                6 => "sixteen".into(),
                7 => "seventeen".into(),
                8 => "eighteen".into(),
                9 => "nineteen".into(),
                _ => unreachable!(),
            }
        }
        (2, _) => "twenty".into(),
        (3, _) => "thirty".into(),
        (4, _) => "forty".into(),
        (5, _) => "fifty".into(),
        (6, _) => "sixty".into(),
        (7, _) => "seventy".into(),
        (8, _) => "eighty".into(),
        (9, _) => "ninety".into(),
        _ => unreachable!(),
    };
    match n % 10 {
        0 => (),
        u => {
            base.push('-');
            base.push_str(unit(u));
        }
    }
    base
}

fn unit(n: u64) -> &'static str {
    match n {
        0 => "",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => unreachable!(),
    }
}
