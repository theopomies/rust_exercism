use std::iter;

const DIRECTIONS: &[(isize, isize); 4] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
    let matrix = vec![vec![0; size]; size];
    let mut directions = DIRECTIONS.iter().cycle();
    let directions = iter::once(size)
        .chain((1..size).rev().flat_map(|n| iter::repeat(n).take(2)))
        .flat_map(|steps| iter::repeat(directions.next().unwrap()).take(steps));

    (1..=size.pow(2))
        .zip(directions)
        .fold(
            (matrix, (-1, 0)),
            |(mut matrix, (x, y)), (value, direction)| {
                let x = x + direction.0;
                let y = y + direction.1;
                matrix[y as usize][x as usize] = value as u32;
                (matrix, (x, y))
            },
        )
        .0
}
