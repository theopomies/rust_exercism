pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> Luhn for T
where
    T: ToString,
{
    fn valid_luhn(&self) -> bool {
        let code = self
            .to_string()
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
}
