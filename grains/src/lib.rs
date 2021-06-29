pub fn square(s: u32) -> u64 {
    assert!(s >= 1 && s <= 64, "Square must be between 1 and 64");
    1 << (s - 1)
}

pub fn total() -> u64 {
    (1..=64u64)
        .reduce(|acc, curr| acc + square(curr as u32))
        .unwrap()
    // .map(square).sum() could be used to be more declarative
    // but would iterate one more time.
    //
    // Also, mathematicaly this is just:
    // u64::MAX
    // but it kinda defeats the purpose
}
