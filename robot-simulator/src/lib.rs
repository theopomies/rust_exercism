#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

type Position = (i32, i32);

pub struct Robot {
    pos: Position,
    dir: Direction,
}

use Direction::*;

impl Robot {
    pub fn new(x: i32, y: i32, dir: Direction) -> Self {
        Self { pos: (x, y), dir }
    }

    pub fn turn_right(self) -> Self {
        let dir = match self.dir {
            North => East,
            East => South,
            South => West,
            West => North,
        };
        Self { dir, ..self }
    }
    pub fn turn_left(self) -> Self {
        let dir = match self.dir {
            North => West,
            East => North,
            South => East,
            West => South,
        };
        Self { dir, ..self }
    }

    pub fn advance(self) -> Self {
        let (x, y) = self.pos;
        let pos = match self.dir {
            North => (x, y + 1),
            East => (x + 1, y),
            South => (x, y - 1),
            West => (x - 1, y),
        };
        Self { pos, ..self }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |r, i| match i {
            'A' => r.advance(),
            'L' => r.turn_left(),
            'R' => r.turn_right(),
            _ => panic!("Instuctions can only be one of A, L, R"),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.pos
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
