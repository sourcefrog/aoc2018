#![allow(dead_code, unused_imports)]

use std::cmp::{max, min};

use regex::Regex;

use aoc2018::Matrix;
use aoc2018::{point, Point};

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
use self::Thing::*;

impl Thing {
    pub fn is_wet(self) -> bool {
        match self {
            Water | Damp => true,
            Sand | Clay => false,
        }
    }

    /// True if this square can hold water above
    pub fn can_hold(self) -> bool {
        match self {
            Clay | Water => true,
            Sand | Damp => false,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Clay => '#',
            Sand => '.',
            Damp => '|', 
            Water => '~',
        }
    }
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

    fn x_range(ls: &[Line]) -> (usize, usize) {
        let xmin = ls
            .iter()
            .map(|l| match l {
                Line::Horizontal { x1, .. } => x1,
                Line::Vertical { x, .. } => x,
            })
            .min()
            .unwrap();
        let xmax = ls
            .iter()
            .map(|l| match l {
                Line::Horizontal { x2, .. } => x2,
                Line::Vertical { x, .. } => x,
            })
            .max()
            .unwrap();
        (*xmin, *xmax)
    }

    fn y_range(ls: &[Line]) -> (usize, usize) {
        let ymin = ls
            .iter()
            .map(|l| match l {
                Line::Vertical { y1, .. } => y1,
                Line::Horizontal { y, .. } => y,
            })
            .min()
            .unwrap();
        let ymax = ls
            .iter()
            .map(|l| match l {
                Line::Vertical { y2, .. } => y2,
                Line::Horizontal { y, .. } => y,
            })
            .max()
            .unwrap();
        (*ymin, *ymax)
    }
}

pub struct Map {
    m: Matrix<Thing>,
    drip: Vec<Point>,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
}

impl Map {
    fn from_lines(ls: &[Line]) -> Map {
        let (x_min, x_max) = Line::x_range(ls);
        let (y_min, y_max) = Line::y_range(ls);
        let mut m = Matrix::new(x_max + 2, y_max + 2, Sand);
        for l in ls.iter() {
            match *l {
                Line::Vertical { x, y1, y2 } => {
                    for y in y1..=y2 {
                        m[point(x, y)] = Clay;
                    }
                }
                Line::Horizontal { x1, x2, y } => {
                    for x in x1..=x2 {
                        m[point(x, y)] = Clay;
                    }
                }
            }
        }
        println!(
            "Created map; xrange={}..={}, yrange={}..={}",
            x_min, x_max, y_min, y_max
        );
        // Skip from (500, 0) down to the first point on the map.
        let drip1 = point(500, y_min);
        let mut map = Map {
            m,
            drip: vec![],
            x_min,
            x_max,
            y_min,
            y_max,
        };
        map.add_drip(drip1);
        map
    }

    fn check_point(&self, p: Point) {
        // NB: Any x-coordinate is valid, however it can only overflow one column to the left or right.
        assert!(
            p.x <= (self.x_max + 1) && self.y_min <= p.y && p.y <= self.y_max,
            "Out-of-range point {:?}",
            p
        );
    }

    /// Fill the map until there are no more active drips in range.
    fn run(&mut self) {
        while let Some(drp) = self.drip.pop() {
            println!("drip {:?}", drp);
            self.check_point(drp);
            if drp.y == self.y_max {
                // Falls off the bottom; nothing more to do.
                println!("... falls off the bottom");
                continue;
            }
            match self.m[drp.down()] {
                Damp => {
                    // This isn't an error, as it might occur if there are two paths
                    // that reach the same spot. However, it's not necessary to traverse
                    // it any further because water has already flown through here.
                    println!("already damp; stopping");
                }
                Sand => {
                    // println!("continue down");
                    self.add_drip(drp.down());
                }
                Clay | Water => self.spread(drp),
            }
        }
    }

    fn add_drip(&mut self, p: Point) {
        match self.m[p] {
            Clay => panic!("can't drip through clay at {:?}", p),
            Sand => {
                self.m[p] = Damp;
            }
            Damp => {
                // This square is already damp: perhaps we passed it on the way down, but
                // let's pour in more water and see if it will spread to the sides.
                println!("drip into damp at {:?}", p);
            }
            Water => { 
                // drips into water, but probably nothing to do here
                println!("drip into water at {:?}", p);
            }
        }
        self.drip.push(p);
    }

    /// Spread water horizontally from `drp`, both left and right, until either reaching
    /// clay that will hold it to the side, or finding sand/dampness below where it can 
    /// leak out. If it's enclosed on both sides and below, fill this with water, otherwise
    /// with damp sand. And, if it can leak from either or both sides, create a new drip from
    /// there.
    fn spread(&mut self, drp: Point) {
        println!("water spreads from {:?}", drp);
        let mut pl = drp;
        let mut leak_left = true;
        assert!(self.m[drp] != Clay);
        loop {
            if self.m[pl] == Clay {
                // found a wall; water or dampness fills to pl 
                println!("found left wall of {:?} at {:?}", drp, pl);
                pl = pl.right();
                leak_left = false;
                break;
            } else if self.m[pl.down()].can_hold() {
                // continue across
            } else {
                println!("found left leak below from {:?}", pl);
                self.add_drip(pl.down());
                break;
            }
            pl = pl.left();
        }
        let mut pr = drp;
        let mut leak_right = true;
        while pr.x <= self.x_max {
            if self.m[pr] == Clay {
                // found a wall; water or dampness fills to pr 
                println!("found right wall of {:?} at {:?}", drp, pr);
                pr = pr.left();
                leak_right = false;
                break;
            } else if self.m[pr.down()].can_hold() {
                // continue across
            } else {
                println!("found right leak below from {:?}", pr);
                self.add_drip(pr.down());
                break;
            }
            pr = pr.right();
        }
        if leak_left || leak_right {
            println!("line of dampness from {:?} to {:?}", pl, pr);
            self.fill(Damp, pl, pr);
        } else {
            println!("water holds from {:?} to {:?}", pl, pr);
            self.fill(Water, pl, pr);
            // continue pouring in water, one level higher
            if drp.y > self.y_min {
                self.add_drip(drp.up())
            }
        }
    }

    fn fill(&mut self, th: Thing, p1: Point, p2: Point) {
        assert_eq!(p1.y, p2.y);
        for x in min(p1.x, p2.x)..=max(p1.x, p2.x) {
            let p = point(x, p1.y);
            assert!(self.m[p] != Clay);
            self.m[p] = th;
        }
    }

    fn count_wet(&self) -> usize {
        self.m.values().filter(|t| t.is_wet()).count()
    }

    fn count_water(&self) -> usize {
        self.m.values().filter(|t| **t == Water).count()
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for y in 0..=self.y_max {
            for x in (self.x_min-1)..=(self.x_max+1) {
                s.push(self.m[point(x, y)].to_char());
            }
            s.push('\n')
        }
        s
    }
}

/// Solve the main puzzle.
pub fn solve_main_input() -> (usize, usize) {
    use std::io::{Read, Write};
    use std::fs::File;
    let mut s = String::new();
    File::open("input/input17.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    solve_str(&s)
}

/// Solve the puzzle in the given representation
pub fn solve_str(s: &str) -> (usize, usize) {
    let mut map = Map::from_lines(&Line::parse_lines(&s));
    // write!(File::create("aoc17before.txt").unwrap(), "{}", map.render()).unwrap();
    map.run();
    // write!(File::create("aoc17after.txt").unwrap(), "{}", map.render()).unwrap();
    (map.count_wet(), map.count_water())
}

pub fn main() {
    let (n_wet, n_water) = solve_main_input();
    println!("wet squares: {}", n_wet);
    println!("water squares: {}", n_water);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
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
        assert_eq!(Line::y_range(&ls), (1, 13));

        let mut map = Map::from_lines(&ls);
        assert_eq!(map.m[point(495, 2)], Clay);
        assert_eq!(map.m[point(495, 7)], Clay);

        map.run();
        assert_eq!(57, map.count_wet());
    }

    #[test]
    fn real_problem() {
        assert_eq!(solve_main_input(), (33052, 27068));
    }
}
