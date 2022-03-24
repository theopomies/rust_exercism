use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match (1..=num / 2)
        .filter(|divisor| num % divisor == 0)
        .sum::<u64>()
        .cmp(&num)
    {
        _ if num == 0 => None,
        Ordering::Greater => Some(Classification::Abundant),
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Less => Some(Classification::Deficient),
    }
}
