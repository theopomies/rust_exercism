pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|value| {
            factors
                .iter()
                .any(|&factor| factor != 0 && value.rem_euclid(factor) == 0)
        })
        .sum()
}
