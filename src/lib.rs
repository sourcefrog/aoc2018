pub mod matrix;
pub use crate::matrix::Matrix;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Point {
    pub y: usize,
    pub x: usize,
}

/// Shorthand to construct a point.
pub fn point(x: usize, y: usize) -> Point {
    Point { x, y }
}
