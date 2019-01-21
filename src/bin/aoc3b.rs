/// https://adventofcode.com/2018/day/3
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::BTreeSet;
use std::io;
use std::io::prelude::*;
use std::iter::Iterator;

use regex::Regex;

pub fn main() {
    let cls = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|s| Claim::from_string(&s))
        .collect::<Vec<_>>();
    println!(
        "square with no overlap: {}",
        find_no_overlaps(&cls).unwrap()
    );
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl Claim {
    pub fn from_string(s: &str) -> Claim {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        let ci = |i| caps.get(i).unwrap().as_str().parse().unwrap();
        Claim {
            id: ci(1),
            x: ci(2),
            y: ci(3),
            w: ci(4),
            h: ci(5),
        }
    }

    /// Return set of (x,y) square coordinates occupied.
    pub fn squares(&self) -> BTreeSet<(u32, u32)> {
        let mut h = BTreeSet::new();
        for x in self.x..(self.x + self.w) {
            for y in self.y..(self.y + self.h) {
                h.insert((x, y));
            }
        }
        h
    }
}

/// Return the id of the first square that has no overlaps with anything else.
fn find_no_overlaps(cls: &[Claim]) -> Option<u32> {
    // Set of squares claimed at least once
    let mut once = BTreeSet::<(u32, u32)>::new();
    // Set of squares claimed at least twice
    let mut twice = BTreeSet::<(u32, u32)>::new();
    for c in cls {
        for s in c.squares() {
            if !once.insert(s) {
                twice.insert(s);
            }
        }
    }
    for c in cls {
        if twice.is_disjoint(&c.squares()) {
            return Some(c.id);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_claim() {
        assert_eq!(
            Claim::from_string("#1 @ 1,3: 4x4"),
            Claim {
                id: 1,
                x: 1,
                y: 3,
                w: 4,
                h: 4
            }
        );
        assert_eq!(
            Claim::from_string("#2 @ 3,1: 4x4"),
            Claim {
                id: 2,
                x: 3,
                y: 1,
                w: 4,
                h: 4
            }
        );
        assert_eq!(
            Claim::from_string("#3 @ 5,5: 2x2"),
            Claim {
                id: 3,
                x: 5,
                y: 5,
                w: 2,
                h: 2
            }
        );
    }

    #[test]
    fn test_overlaps_one_claim() {
        assert_eq!(
            find_no_overlaps(&[Claim::from_string("#1 @ 1,3: 4x4"),]),
            Some(1)
        );
    }

    #[test]
    fn test_overlaps_three_claims() {
        assert_eq!(
            find_no_overlaps(&[
                Claim::from_string("#1 @ 1,3: 4x4"),
                Claim::from_string("#2 @ 3,1: 4x4"),
                Claim::from_string("#3 @ 5,5: 2x2"),
            ]),
            Some(3)
        );
    }
}
