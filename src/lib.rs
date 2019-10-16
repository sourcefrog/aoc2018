//! mbp AoC2018 solutions - general utilities.

pub mod bisection_search;
pub mod matrix;
mod point;
mod shortest_path;

pub use crate::bisection_search::bisection_search;
pub use crate::matrix::Matrix;
pub use crate::point::{point, Point};
pub use crate::shortest_path::shortest_distance;
