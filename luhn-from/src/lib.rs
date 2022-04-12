pub struct Luhn(bool);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.0
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        let code = input
            .to_string()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        if code.len() < 2 {
            return Self(false);
        }
        let parity = code.len() % 2;

        let is_valid = code
            .chars()
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
            == 0;
        Self(is_valid)
    }
}
