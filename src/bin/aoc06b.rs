// Copyright 2018 Google LLC
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     https://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! https://adventofcode.com/2018/day/6
//!
//! For every point, calculate the distance to every landing, stopping if
//! we get above the limit. If we complete before getting to the limit,
//! that point counts.

use std::io;
use std::io::prelude::*;

pub fn main() {
    let pts: Vec<Point> = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|s| Point::from_string(&s))
        .collect();
    let m = Map::from_points(pts);
    const N: i32 = 10_000;
    println!("largest within {}: {}", N, m.count_within_distance(N));
}

type Coord = i32;
#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: Coord,
    y: Coord,
}

#[derive(Clone, Debug, PartialEq)]
struct Map {
    // For simplicity addressing is zero-based even though that may leave
    // some empty space to the top-left.
    w: Coord,
    h: Coord,
    ls: Vec<Point>,
}

impl Point {
    pub fn from_string(s: &str) -> Point {
        let mut splits = s.split(", ");
        Point {
            x: splits.next().unwrap().parse().unwrap(),
            y: splits.next().unwrap().parse().unwrap(),
        }
    }

    fn abs_difference(&self, other: &Point) -> Coord {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Map {
    /// Make a new map that will fit all these points
    pub fn from_points(points: Vec<Point>) -> Map {
        Map {
            w: points.iter().map(|p| p.x).max().unwrap() + 2,
            h: points.iter().map(|p| p.y).max().unwrap() + 2,
            ls: points,
        }
    }

    fn count_within_distance(&self, limit: i32) -> u32 {
        let mut n = 0;
        for y in 0..self.h {
            for x in 0..self.w {
                let mut t = 0;
                let p = Point { x, y };
                for l in &self.ls {
                    t += p.abs_difference(l);
                    if t > limit {
                        break;
                    }
                }
                if t < limit {
                    n += 1;
                }
            }
        }
        n
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let pts: Vec<_> = [(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)]
            .iter()
            .map(|(x, y)| Point { x: *x, y: *y })
            .collect();
        let m = Map::from_points(pts);
        println!("{:?}", &m);

        assert_eq!(m.count_within_distance(32), 16);
    }
}
