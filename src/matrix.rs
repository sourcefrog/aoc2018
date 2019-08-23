//! A rectangular 2d matrix.
//!
//! Matrices are indexed by (row, column) coordinates.
use std::ops::{Index, IndexMut};

use crate::{point, Point};

#[derive(Clone, Eq, PartialEq)]
pub struct Matrix<T> {
    w: usize,
    h: usize,
    d: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    pub fn new(w: usize, h: usize, d: T) -> Matrix<T> {
        Matrix {
            w,
            h,
            d: vec![d; w * h],
        }
    }

    /// Make a builder that will accumulate rows of a matrix.
    pub fn from_rows() -> FromRows<T> {
        FromRows::<T> {
            w: 0,
            d: Vec::new(),
        }
    }

    pub fn width(&self) -> usize {
        self.w
    }

    pub fn height(&self) -> usize {
        self.h
    }

    /// Return all values in row,col order.
    pub fn values(&self) -> std::slice::Iter<'_, T> {
        self.d.iter()
    }

    /// Return a vec of all present 8-way neighbors.
    pub fn neighbor8_values(&self, p: Point) -> Vec<T> {
        let mut v: Vec<T> = Vec::with_capacity(8);
        if p.y > 0 {
            if p.x > 0 {
                v.push(self[p.left().up()].clone())
            }
            v.push(self[p.up()].clone());
            if p.x < self.w - 1 {
                v.push(self[p.right().up()].clone())
            }
        }
        if p.x > 0 {
            v.push(self[p.left()].clone())
        }
        if p.x < self.w - 1 {
            v.push(self[p.right()].clone())
        }
        if p.y < self.h - 1 {
            if p.x > 0 {
                v.push(self[p.left().down()].clone())
            }
            v.push(self[p.down()].clone());
            if p.x < self.w - 1 {
                v.push(self[p.right().down()].clone())
            }
        }
        v
    }

    pub fn iter_points<'a>(&'a self) -> Box<dyn Iterator<Item = Point> + 'a> {
        Box::new((0..self.h).flat_map(move |y| (0..self.w).map(move |x| point(x, y))))
    }
}

impl<T: Clone> Index<Point> for Matrix<T> {
    type Output = T;
    fn index(&self, p: Point) -> &T {
        &self.d[self.w * p.y + p.x]
    }
}

impl<T: Clone> IndexMut<Point> for Matrix<T> {
    fn index_mut(&mut self, p: Point) -> &mut T {
        assert!(p.x < self.w, "{:?} too wide for {}", p, self.w);
        assert!(p.y < self.h);
        &mut self.d[self.w * p.y + p.x]
    }
}

pub struct FromRows<T> {
    w: usize,
    d: Vec<T>,
}

impl<T: Clone> FromRows<T> {
    pub fn add_row(&mut self, r: &[T]) {
        if self.d.is_empty() {
            // First row
            assert!(!r.is_empty());
            self.w = r.len();
            self.d.extend_from_slice(r);
        } else {
            assert_eq!(r.len(), self.w, "Rows must be the same length");
            self.d.extend_from_slice(r);
        }
    }

    pub fn finish(mut self) -> Matrix<T> {
        self.d.shrink_to_fit();
        assert!(self.d.len() % self.w == 0, "Matrix isn't rectangular");
        Matrix {
            w: self.w,
            h: self.d.len() / self.w,
            d: self.d,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_matrix() {
        let mut m = Matrix::new(10, 10, 7u8);
        assert_eq!(m[point(5, 5)], 7u8);
        m[point(6, 6)] = 10;
        assert_eq!(m[point(6, 6)], 10);
        assert_eq!(m[point(5, 5)], 7u8);
    }

    #[test]
    fn from_rows() {
        let mut b = Matrix::from_rows();
        b.add_row(&[1, 2, 3]);
        b.add_row(&[4, 5, 6]);
        b.add_row(&[7, 8, 9]);
        let m = b.finish();
        assert_eq!(m.width(), 3);
        assert_eq!(m.height(), 3);
        assert_eq!(m[point(0, 0)], 1);
        assert_eq!(m[point(2, 0)], 3);
        assert_eq!(m[point(2, 2)], 9);
    }
}
