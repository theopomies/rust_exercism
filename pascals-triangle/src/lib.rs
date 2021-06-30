pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}
// Very Elegant solution, reminds me of Haskell
//
// use std::iter;
// let rows = match row_count {
//     0 => vec![],
//     1 => vec![vec![1]],
//     _ => {
//         let mut rows = Self::new(row_count - 1).rows;
//         let new_row = {
//             let last_row = rows.last().unwrap();
//             let lhs = iter::once(0).chain(last_row.iter().cloned());
//             let rhs = last_row.iter().cloned().chain(iter::once(0));
//             lhs.zip(rhs).map(|(l, r)| l + r).collect()
//         };
//         rows.push(new_row);
//         rows
//     }
// };
impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let rows = (1usize..=row_count as usize)
            .scan(vec![1u32], |state, idx| {
                state.insert(0, 0);
                state.push(0);
                *state = (0usize..idx)
                .map(|value| *state.get(value).unwrap() + *state.get(value + 1).unwrap())
                .collect();
                Some(state.clone())
            })
            .collect();
        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
