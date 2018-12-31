#![allow(dead_code)]

use std::ops::RangeInclusive;

use regex::Regex;

// Read the input lines and draw into a matrix. Maybe pre-scan to work out the
// maximum dimensions.
//
// Iterate from each "drip" until reaching a stable state.
//
// If the square below the drip is sand, it simply falls down.
//
// If the square below is water or clay, try to spread sideways.
//
// It shouldn't happen that the square below is just damp.

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Thing {
    Sand,
    Clay,
    Water,
    Damp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Line {
    Vertical { x: usize, y1: usize, y2: usize },
    Horizontal { y: usize, x1: usize, x2: usize },
}

impl Line {
    fn parse_lines(s: &str) -> Vec<Line> {
        let mut v = Vec::new();
        let v_re = Regex::new(r"x=([0-9]+), y=([0-9]+)\.\.([0-9]+)").unwrap();
        let h_re = Regex::new(r"y=([0-9]+), x=([0-9]+)\.\.([0-9]+)").unwrap();
        for caps in v_re.captures_iter(s) {
            v.push(Line::Vertical {
                x: caps[1].parse().unwrap(),
                y1: caps[2].parse().unwrap(),
                y2: caps[3].parse().unwrap(),
            });
        }
        for caps in h_re.captures_iter(s) {
            v.push(Line::Horizontal {
                y: caps[1].parse().unwrap(),
                x1: caps[2].parse().unwrap(),
                x2: caps[3].parse().unwrap(),
            });
        }
        v
    }

    fn y_range(ls: &[Line]) -> RangeInclusive<usize> {
        let ymin = ls.iter().map(|l| match l {
            Line::Vertical { y1, .. } => y1,
            Line::Horizontal { y, .. } => y,
        }).min().unwrap();
        let ymax = ls.iter().map(|l| match l {
            Line::Vertical { y2, .. } => y2,
            Line::Horizontal { y, .. } => y,
        }).max().unwrap();
        RangeInclusive::new(*ymin, *ymax)
    }
}

pub fn solve() {
    use std::io::Read;
    let mut s = String::new();
    std::fs::File::open("input/input17.txt").unwrap().read_to_string(&mut s).unwrap();
    let _lines = Line::parse_lines(&s);
}

pub fn main() {
    solve();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let ls = Line::parse_lines(
            "
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
",
        );
        assert_eq!(ls.len(), 8);
        assert_eq!(
            ls[0],
            Line::Vertical {
                x: 495,
                y1: 2,
                y2: 7
            }
        );
        assert_eq!(Line::y_range(&ls), RangeInclusive::new(1, 13));
    }
}
