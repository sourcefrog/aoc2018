#![allow(dead_code)]

/// https://adventofcode.com/2018/day/22

use aoc2018::{point, Point};

use std::collections::{BTreeMap};

type Erosion = usize;

enum Tool {
    Climbing,
    Torch,
    None,
}

enum Ground {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}

impl Ground {
    pub fn from_int(g: usize) -> Ground {
        match g {
            0 => Ground::Rocky,
            1 => Ground::Wet,
            2 => Ground::Narrow,
            _ => panic!("unexpected ground value"),
        }
    }
}

struct Map {
    /// Memoized ground type.
    g: BTreeMap<Point, Ground>,

    /// Memoized erosion levels.
    e: BTreeMap<Point, Erosion>,

    depth: usize,

    target: Point,
}

impl Map {
    fn new(depth: usize, target: Point) -> Map {
        Map {
            g: Default::default(),
            e: Default::default(),
            depth,
            target,
        }
    }

    fn erosion_at(&mut self, p: Point) -> Erosion {
        if let Some(e) = self.e.get(&p) {
            return *e;
        }
        let v = if p == self.target {
            0
        } else if p.y == 0 {
            // This also handles the (0,0) case.
            p.x.checked_mul(16807).unwrap()
        } else if p.x == 0 {
            p.y.checked_mul(48271).unwrap()
        } else {
            let v1 = self.erosion_at(p.left());
            let v2 = self.erosion_at(p.up());
            v1.checked_mul(v2).unwrap()
        };
        let e = (v + self.depth) % 20183;
        self.e.insert(p, e);
        e
    }

    pub fn ground_at(&mut self, p: Point) -> Ground {
        Ground::from_int(self.erosion_at(p) % 3)
    }

    pub fn calc_risk(&mut self) -> usize {
        // The modulus calculation of region type exactly corresponds to the
        // risk of each region: 0=rocky, 1=wet, 2=narrow.
        let mut sum = 0;
        for x in 0..=self.target.x {
            for y in 0..=self.target.y {
                sum += self.erosion_at(point(x, y)) % 3
            }
        }
        sum
    }
}

pub fn solve() -> usize {
    Map::new(5616, point(10, 785)).calc_risk()
}

pub fn main() {
    println!("Result: {}", solve());;
}

#[cfg(test)]
mod test {
    use super::Map;
    use aoc2018::point;

    #[test]
    fn build_map() {
        let mut map = Map::new(510, point(10, 10));
        assert_eq!(map.erosion_at(point(0, 0)), 510);
        assert_eq!(map.erosion_at(point(1, 0)), 17317);
        assert_eq!(map.erosion_at(point(0, 1)), 8415);
        assert_eq!(map.erosion_at(point(1, 1)), 1805);
        assert_eq!(map.erosion_at(point(10, 10)), 510);

        assert_eq!(map.calc_risk(), 114);
    }

    #[test]
    fn expected_solution() {
        assert_eq!(super::solve(), 8681);
    }
}
