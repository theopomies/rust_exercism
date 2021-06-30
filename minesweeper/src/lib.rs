use std::cmp::min;
use std::ops::RangeInclusive;

fn get_checking_range(idx: usize, len: usize) -> RangeInclusive<usize> {
    let low = match idx.checked_sub(1) {
        Some(idx) => idx,
        None => idx,
    };
    let high = match idx.checked_add(1) {
        Some(idx) => min(idx, len - 1),
        None => idx,
    };
    low..=high
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut annotated_minefield = Vec::with_capacity(minefield.len());

    for (x, row) in minefield.iter().enumerate() {
        annotated_minefield.push(String::with_capacity(row.len()));
        for (y, c) in row.chars().enumerate() {
            if c == '*' {
                annotated_minefield[x].push('*');
                continue;
            }
            let mut count = 0;
            let x_range = get_checking_range(x, minefield.len());
            let y_range = get_checking_range(y, row.len());
            for new_x in x_range {
                for new_y in y_range.clone() {
                    if new_x == x && new_y == y {
                        continue;
                    }
                    if let Some("*") = minefield[new_x].get(new_y..=new_y) {
                        count += 1;
                    }
                }
            }
            match count {
                0 => annotated_minefield[x].push(' '),
                _ => annotated_minefield[x].push(char::from_digit(count, 10).unwrap()),
            }
        }
    }
    annotated_minefield
}
