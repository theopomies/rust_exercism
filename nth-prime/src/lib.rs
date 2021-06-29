use std::f64;

fn is_prime(n: u32) -> bool {
    !(2..(n as f64).sqrt().floor() as u32 + 1).any(|i| n % i == 0)
}

pub fn nth(n: u32) -> u32 {
    (2..).filter(|&num| is_prime(num)).nth(n as usize).unwrap()
}
