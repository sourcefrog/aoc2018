pub mod matrix;
pub use crate::matrix::Matrix;

use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Point {
    pub y: usize,
    pub x: usize,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "point({}, {})", self.x, self.y)
    }
}

/// Shorthand to construct a point.
pub fn point(x: usize, y: usize) -> Point {
    Point { x, y }
}
