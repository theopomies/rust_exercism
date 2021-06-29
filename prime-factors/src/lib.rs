pub fn factors(mut n: u64) -> Vec<u64> {
    let mut prime_factors = vec![];

    for candidate in 2u64.. {
        if n == 1 {
            break;
        }
        while n % candidate == 0 {
            prime_factors.push(candidate);
            n /= candidate;
        }
    }
    prime_factors
}
