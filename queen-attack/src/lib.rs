#[warn(clippy::all, clippy::pedantic)]
#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Self(rank, file)),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let ChessPosition(rs, fs) = self.0;
        let ChessPosition(ro, fo) = other.0;
        rs == ro || fs == fo || (ro - rs).abs() == (fo - fs).abs()
    }
}
