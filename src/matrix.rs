/// A rectangular 2d matrix.
///
/// Matrices are indexed by (row, column) coordinates.
use std::ops::{Index, IndexMut};

use crate::Point;

#[derive(Eq, PartialEq)]
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
}

impl<T: Clone> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, p: (usize, usize)) -> &T {
        &self.d[self.w * p.0 + p.1]
    }
}

impl<T: Clone> Index<Point> for Matrix<T> {
    type Output = T;
    fn index(&self, p: Point) -> &T {
        &self.d[self.w * p.y + p.x]
    }
}

impl<T: Clone> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, p: (usize, usize)) -> &mut T {
        assert!(p.0 < self.h);
        assert!(p.1 < self.w);
        &mut self.d[self.w * p.0 + p.1]
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
        assert_eq!(m[(5, 5)], 7u8);
        m[(6, 6)] = 10;
        assert_eq!(m[(6, 6)], 10);
        assert_eq!(m[(5, 5)], 7u8);
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
        assert_eq!(m[(0, 0)], 1);
        assert_eq!(m[(0, 2)], 3);
        assert_eq!(m[(2, 2)], 9);
    }
}
