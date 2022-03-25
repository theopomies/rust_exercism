pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            let row_max = row.iter().max();
            row.iter().enumerate().filter_map(move |(col_idx, value)| {
                row_max.and_then(|max| (value == max).then(|| (row_idx, col_idx, *value)))
            })
        })
        .filter_map(|(row_idx, col_idx, value)| {
            let col_min = input.iter().map(|row| row[col_idx]).min()?;
            (value == col_min).then(|| (row_idx, col_idx))
        })
        .collect()
}
