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

#![allow(dead_code)]

//! https://adventofcode.com/2018/day/12
//!
//! Pots are simply binary.
//!
//! Evolves in generations rather than updating in place.
//!
//! The furthest it can possibly propagate out to the left or right is
//! two pots per generation.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::ops::Range;
use std::rc::Rc;

pub fn main() {
    let mut s = String::new();
    File::open("input/input12.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let mut p: Pots = s.parse().unwrap();
    for _i in 0..20 {
        p = p.step();
    }
    println!("result = {}", p.magic());
}

fn from_b(c: u8) -> bool {
    match c {
        b'#' => true,
        b'.' => false,
        e => panic!("unexpected {:?}", e),
    }
}

#[derive(Clone)]
struct Pots {
    /// Indices of pots that are occupied.
    pots: BTreeSet<isize>,
    /// Map of instructions from 5-bool context to new results
    inst: Rc<BTreeMap<[bool; 5], bool>>,
}

impl fmt::Debug for Pots {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Pots {{ pots={} }}",
            self.format_pots(self.left()..self.right())
        )
    }
}

impl std::str::FromStr for Pots {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let pots = Pots::parse_first_line(lines.next().unwrap());
        let mut inst = BTreeMap::default();
        assert_eq!(lines.next().unwrap(), "");
        for l in lines {
            let lb: &[u8] = l.as_ref();
            let mut bs = [false; 5];
            for i in 0..5 {
                bs[i] = from_b(lb[i]);
            }
            let br = from_b(lb[9]);
            assert_eq!(inst.insert(bs, br), None, "key {:?} already present", bs);
        }
        Ok(Pots {
            pots,
            inst: Rc::new(inst),
        })
    }
}

impl Pots {
    fn parse_first_line(s: &str) -> BTreeSet<isize> {
        let (prefix, bs) = s.split_at(15);
        assert_eq!(prefix, "initial state: ");
        let mut pots = BTreeSet::new();
        for (i, c) in bs.bytes().map(from_b).enumerate() {
            if c {
                pots.insert(i as isize);
            }
        }
        pots
    }

    pub fn set(&mut self, i: isize, b: bool) {
        if b {
            self.pots.insert(i);
        } else {
            self.pots.remove(&i);
        }
    }

    pub fn get(&self, i: isize) -> bool {
        self.pots.contains(&i)
    }

    /// Number of the highest pot that's set
    fn right(&self) -> isize {
        *self.pots.iter().next_back().unwrap()
    }

    /// Number of the lowest pot that's set
    fn left(&self) -> isize {
        *self.pots.iter().next().unwrap()
    }

    pub fn format_pots(&self, r: Range<isize>) -> String {
        let mut s = String::new();
        for i in r {
            s.push(if self.get(i) { '#' } else { '.' });
        }
        s
    }

    /// Return the values of the 5 pots around i
    fn around(&self, i: isize) -> [bool; 5] {
        let mut a = [false; 5];
        for (j, aa) in a.iter_mut().enumerate() {
            *aa = self.get(i + (j as isize) - 2);
        }
        a
    }

    /// Produce new pots for the next step
    pub fn step(&self) -> Pots {
        let mut pots = BTreeSet::new();
        for i in (self.left() - 2)..=(self.right() + 2) {
            let a = self.around(i);
            let n = self.inst.get(&a).unwrap_or(&false);
            if *n {
                pots.insert(i);
            }
        }
        Pots {
            pots,
            inst: self.inst.clone(),
        }
    }

    /// Return the sum of pot-numbers that have a plant.
    pub fn magic(&self) -> isize {
        self.pots.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let p: Pots = "\
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #"
            .parse()
            .unwrap();
        println!("p = {:?}", p);

        assert_eq!(
            p.format_pots(-3..36),
            "...#..#.#..##......###...###..........."
        );
        assert_eq!(p.get(20), false);
        assert_eq!(p.get(22), true);
        assert_eq!(p.around(0), [false, false, true, false, false]);
        assert_eq!(p.around(2), [true, false, false, true, false]);

        let mut p1 = p.step();
        assert_eq!(
            p1.format_pots(-3..36),
            "...#...#....#.....#..#..#..#..........."
        );

        for _i in 2..=20 {
            p1 = p1.step();
        }
        assert_eq!(
            p1.format_pots(-3..36),
            ".#....##....#####...#######....#.#..##."
        );

        assert_eq!(p1.magic(), 325);
    }
}
