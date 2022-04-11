use std::iter::repeat;

pub fn get_diamond(c: char) -> Vec<String> {
    let mid = (c as u8) - b'A';
    let first_half = (0..=mid).map(|idx| {
        repeat(' ')
            .take(mid as usize * 2 + 1)
            .enumerate()
            .map(|(inner_idx, c)| {
                (inner_idx as i64 - mid as i64)
                    .abs()
                    .eq(&(idx as i64))
                    .then(|| (b'A' + idx) as char)
                    .unwrap_or(c)
            })
            .collect()
    });
    first_half.clone().chain(first_half.rev().skip(1)).collect()
}
