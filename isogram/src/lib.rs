use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    candidate
        .to_lowercase()
        .chars()
        .try_fold(
            HashSet::with_capacity(candidate.len()),
            |mut set, c| match (set.insert(c), c.is_alphabetic()) {
                (true, _) | (_, false) => Some(set),
                (false, _) => None,
            },
        )
        .is_some()
}
