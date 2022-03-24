use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match num {
        0 => None,
        _ => Some(
            match (1..num)
                .filter(|divisor| num % divisor == 0)
                .sum::<u64>()
                .cmp(&num)
            {
                Ordering::Greater => Classification::Abundant,
                Ordering::Equal => Classification::Perfect,
                Ordering::Less => Classification::Deficient,
            },
        ),
    }
}
