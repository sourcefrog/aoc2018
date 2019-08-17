//! https://adventofcode.com/2018/day/23
//!
//! Cloud of nanobots able to teleport things within a distance
//! of their (x,y,z) position.

extern crate regex;

use std::fs::File;
use std::io::Read;

use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Bot {
    x: isize,
    y: isize,
    z: isize,
    r: isize,
}

/// Parse an input string containing bot position descriptions into a vec
/// of Bots.
fn parse(s: &str) -> Vec<Bot> {
    let re: Regex = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    s.lines()
        .map(|l| {
            if let Some(caps) = re.captures(l) {
                let fld = |i| caps.get(i).unwrap().as_str().parse().unwrap();
                Bot {
                    x: fld(1),
                    y: fld(2),
                    z: fld(3),
                    r: fld(4),
                }
            } else {
                panic!("failed to parse: {:?}", l);
            }
        })
        .collect()
}

/// Return the strongest Bot, which is the one with the largest radius.
fn strongest(bs: &[Bot]) -> Bot {
    *bs.iter().max_by_key(|b| b.r).unwrap()
}

/// Return the Manhattan distance between two bots.
fn distance(a: &Bot, b: &Bot) -> isize {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
}

/// Return the number of bots in range of the strongest bot (including
/// itself.)
fn count_in_range(bs: &[Bot]) -> usize {
    let st = strongest(&bs);
    bs.iter().filter(|b| distance(&st, b) <= st.r).count()
}

/// Load bots from input file.
fn load_input() -> Vec<Bot> {
    let mut s = String::with_capacity(50_000);
    File::open("input/input23.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    parse(&s)
}

/// Solve part A from real input.
fn solve_a() -> usize {
    count_in_range(&load_input())
}

pub fn main() {
    dbg!(solve_a());
}

#[cfg(test)]
mod tests {
    use super::Bot;

    #[test]
    fn example_1() {
        let t = "\
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1
";
        let bots = super::parse(t);
        assert_eq!(bots.len(), 9);
        assert_eq!(
            bots[0],
            Bot {
                x: 0,
                y: 0,
                z: 0,
                r: 4
            }
        );
        assert_eq!(
            bots[8],
            Bot {
                x: 1,
                y: 3,
                z: 1,
                r: 1,
            }
        );
        assert_eq!(
            super::strongest(&bots),
            Bot {
                x: 0,
                y: 0,
                z: 0,
                r: 4,
            }
        );
        assert_eq!(super::count_in_range(&bots), 7);
    }

    #[test]
    fn expected_result() {
        assert_eq!(super::solve_a(), 232);
    }
}
