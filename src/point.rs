//! Simple 2D integer-indexed point.
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

impl Point {
    pub fn down(&self) -> Point {
        point(self.x, self.y.checked_add(1).unwrap())
    }

    pub fn left(&self) -> Point {
        point(self.x.checked_sub(1).unwrap(), self.y)
    }

    pub fn right(&self) -> Point {
        point(self.x.checked_add(1).unwrap(), self.y)
    }

    pub fn up(&self) -> Point {
        point(self.x, self.y.checked_sub(1).unwrap())
    }

    pub fn neighbors(&self) -> Vec<Point> {
        let mut r = Vec::with_capacity(4);
        if self.x > 0 {
            r.push(self.left())
        }
        if self.y > 0 {
            r.push(self.up())
        }
        r.push(self.right());
        r.push(self.down());
        r
    }
}
