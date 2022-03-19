use std::collections::HashSet;

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Self> {
        let mut r = 0;
        let mut v = value;
        while v > 0 {
            r = r * 10 + v % 10;
            v /= 10;
        }

        if value == r {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut products = HashSet::new();
    for lhs in min..=max {
        for rhs in lhs..=max {
            if let Some(Palindrome(product)) = Palindrome::new(lhs * rhs) {
                products.insert(product);
            }
        }
    }
    let mut products: Vec<Palindrome> = products.into_iter().map(|p| Palindrome(p)).collect();

    products.sort_by_key(|p| p.into_inner());
    Some((*products.first()?, *products.last()?))
}
