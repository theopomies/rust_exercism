use std::ops::Add;
use Triangle::*;

#[derive(PartialEq)]
pub enum Triangle {
    Equilateral,
    Isosceles,
    Scalene,
}

impl Triangle {
    pub fn build<T>(sides: [T; 3]) -> Option<Self>
    where
        T: Copy + PartialOrd + Add<Output = T>,
    {
        match sides {
            [v1, v2, v3] if v1 >= v2 + v3 || v2 >= v1 + v3 || v3 >= v1 + v2 => None,
            [v1, v2, v3] if v1 == v2 && v2 == v3 => Some(Equilateral),
            [v1, v2, v3] if v1 == v2 || v2 == v3 || v1 == v3 => Some(Isosceles),
            _ => Some(Scalene),
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self == &Equilateral
    }

    pub fn is_scalene(&self) -> bool {
        self == &Scalene
    }

    pub fn is_isosceles(&self) -> bool {
        self == &Isosceles
    }
}
