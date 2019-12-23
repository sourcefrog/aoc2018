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

//! Infer the terrain of a cave, and then find the shortest path through it.
//!
//! https://adventofcode.com/2018/day/22

use std::collections::BTreeMap;

use aoc2018::shortest_distance;
use aoc2018::{point, Point};

type Erosion = usize;

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Tool {
    Climbing,
    Torch,
    Neither,
}
use Tool::*;

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Ground {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}
use Ground::*;

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
    /// Memoized erosion levels.
    e: BTreeMap<Point, Erosion>,

    depth: usize,

    target: Point,
}

/// Combination of a location, and a tool.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, Eq, PartialEq)]
struct State {
    p: Point,
    t: Tool,
}

impl Map {
    fn new(depth: usize, target: Point) -> Map {
        Map {
            e: BTreeMap::new(),
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
            p.x * 16807
        } else if p.x == 0 {
            p.y * 48271
        } else {
            let v1 = self.erosion_at(p.left());
            let v2 = self.erosion_at(p.up());
            v1 * v2
        };
        let eros = (v + self.depth) % 20183;
        self.e.insert(p, eros);
        eros
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

    /// Return a vec of neighboring states.
    ///
    /// The result includes:
    /// switching to a different permitted tool and staying in the same
    /// place,
    /// or moving to a directly neighboring position compatible with the current
    /// tool.
    fn neighbors(&mut self, st: State) -> Vec<(State, isize)> {
        let mut r = Vec::with_capacity(5);
        let new_tool = match (self.ground_at(st.p), st.t) {
            (Rocky, Climbing) => Torch,
            (Rocky, Torch) => Climbing,
            (Wet, Climbing) => Neither,
            (Wet, Neither) => Climbing,
            (Narrow, Torch) => Neither,
            (Narrow, Neither) => Torch,
            (g, t) => panic!("illegal existing state {:?}, {:?}", g, t),
        };
        r.push((
            State {
                t: new_tool,
                p: st.p,
            },
            7,
        ));

        let t = st.t;
        r.extend(
            st.p.neighbors()
                .iter()
                .filter(|np| legal(t, self.ground_at(**np)))
                .map(|np| (State { t, p: *np }, 1)),
        );

        r
    }

    /// Find the target, walking the shortest path, taking account of tool
    /// transitions. Returns the time taken.
    fn rescue(&mut self) -> isize {
        let start = State {
            p: point(0, 0),
            t: Torch,
        };
        let dest = State {
            p: self.target,
            t: Torch,
        };
        // shortest_distance(start, dest, &mut |state| self.neighbors(state))
        shortest_distance(start, dest, &mut |state| self.neighbors(state))
    }
}

/// True if tool `t` is allowed in on ground `g`.
fn legal(t: Tool, g: Ground) -> bool {
    match (g, t) {
        (Rocky, Climbing)
        | (Rocky, Torch)
        | (Wet, Climbing)
        | (Wet, Neither)
        | (Narrow, Torch)
        | (Narrow, Neither) => true,
        _ => false,
    }
}

pub fn solve() -> (usize, isize) {
    let mut m = Map::new(5616, point(10, 785));
    (m.calc_risk(), m.rescue())
}

pub fn main() {
    let (a, b) = solve();
    println!("Part A: {}", a);
    println!("Part B: {}", b);
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
    fn correct_solution() {
        assert_eq!(super::solve(), (8681, 1070));
    }

    #[test]
    fn example_walk() {
        let mut map = Map::new(510, point(10, 10));
        assert_eq!(map.rescue(), 45);
    }
}
