/// https://adventofcode.com/2018/day/6

// Read the list of coordinates; assign each an index.
// Make a map and insert the central coordinates as 0 distance from
// themselves.
//
// The dimensions of the map are a bounding box sufficient to include
// all the points.
//
// Each point can be known or unknown. If it's known, it is either closer
// to one particular point, or it's equidistant from two or more points.
// In rounds:
//
// For each unoccupied square, if it's next to any occupied squares: if
// all of them have the same landing point, this also has that as the
// nearest landing point. If there is more than one different nearest
// landing point, then complete that. All updates to the map are applied
// only after they've all been computed.
//
// Once there are no more squares to be filled, eliminate any areas that
// continue to infinity, which is any ones that are on the outside borders
// of the map.
//
// Finally, count which landing point has the most squares and hasn't been
// eliminated.

use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::prelude::*;

pub fn main() {
    let pts: Vec<Point> = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|s| Point::from_string(&s))
        .collect();
    let m = Map::from_points(&pts).grow_completely();
    println!("largest: {}", m.largest());
}

type Coord = i32;
struct Point {
    x: Coord,
    y: Coord,
}
type Landing = u32;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Color {
    Unknown,
    One(Landing),
    Many,
}

#[derive(Clone, PartialEq)]
struct Map {
    // For simplicity addressing is zero-based even though that may leave
    // some empty space to the top-left.
    w: Coord,
    h: Coord,
    d: Vec<Color>,
}

impl Point {
    pub fn from_string(s: &str) -> Point {
        let mut splits = s.split(", ");
        Point {
            x: splits.next().unwrap().parse().unwrap(),
            y: splits.next().unwrap().parse().unwrap(),
        }
    }

    fn up(&self) -> Point {
        Point{x: self.x, y: self.y-1}
    }

    fn down(&self) -> Point {
        Point{x: self.x, y: self.y+1}
    }

    fn left(&self) -> Point {
        Point{x: self.x-1, y: self.y}
    }

    fn right(&self) -> Point {
        Point{x: self.x+1, y: self.y}
    }
}

impl Map {
    pub fn new(w: Coord, h: Coord) -> Map {
        Map {
            w,
            h,
            d: vec![Color::Unknown; w as usize * h as usize],
        }
    }

    /// Make a new map that will fit all these points
    pub fn from_points(points: &[Point]) -> Map {
        let mut m = Map::new(points.iter().map(|p| p.x).max().unwrap() + 2,
            points.iter().map(|p| p.y).max().unwrap() + 2);
        for (i, p) in points.iter().enumerate() {
            m.set(p, Color::One(i as Landing));
        }
        m
    }

    pub fn set(&mut self, p: &Point, c: Color) {
        let i = self.idx(p);
        assert_eq!(self.get(p), Color::Unknown);
        self.d[i] = c
    }

    pub fn get(&self, p: &Point) -> Color {
        self.d[self.idx(p)]
    }

    pub fn get_or_unknown(&self, p: &Point) -> Color {
        if p.x < 0 || p.x >= self.w || p.y < 0 || p.y >= self.h {
            Color::Unknown
        } else {
            self.get(p)
        }
    }

    fn idx(&self, p: &Point) -> usize {
        assert!(p.x < self.w);
        assert!(p.x >= 0);
        assert!(p.y < self.h);
        assert!(p.y >= 0);
        (p.y * self.w + p.x) as usize
    }

    /// Fill in all squares directly neighboring a new square, returning a
    /// new updated map.
    fn grow(&self) -> Map {
        let mut new = Map::new(self.w, self.h);
        for y in 0..self.h {
            for x in 0..self.w {
                let p = Point{x, y};
                new.set(&p, match self.get(&p) {
                    Color::Unknown => self.grow_one(&p),
                    c => c,
                });
            }
        }
        new
    }

    /// Get the new color for one currently-unknown square
    fn grow_one(&self, p: &Point) -> Color {
        let n = self.neighbors(p);
        if n.iter().any(|&c| c == Color::Many) {
            // If there's any neighbor that's equidistant from multiple landings,
            // then this is too.
            Color::Many
        } else {
            // Find the known colored neighbors. If they're all the same
            // color, this square is too; otherwise there are many nearest
            // neighbors.
            let nc: Vec<&Color> = n.iter().filter(|&c| *c != Color::Unknown).collect();
            if nc.is_empty() {
                Color::Unknown
            } else if nc.iter().all(|c| *c == nc[0]) {
                *nc[0]
            } else {
                Color::Many
            }
        }
    }

    /// Keep growing until the map is stable/full; then return the result.
    fn grow_completely(self) -> Map {
        let mut m = self;
        loop {
            let m2 = m.grow();
            if m2 == m {
                return m;
            }
            m = m2;
        }
    }

    // Collect all neighbors, or Unknown if they're off the map
    fn neighbors(&self, p: &Point) -> [Color; 4] {
        [
            self.get_or_unknown(&p.up()),
            self.get_or_unknown(&p.down()),
            self.get_or_unknown(&p.left()),
            self.get_or_unknown(&p.right()),
        ]
    }

    /// Return landings that are on the border of the map and can continue
    /// outwards to infinite extent.
    pub fn escapees(&self) -> HashSet<Landing> {
        let mut e = HashSet::new();
        // TODO: Is it ever possible to escape through a Many? I think not.
        fn g(m: &Map, e: &mut HashSet<Landing>, p: &Point) {
            if let Color::One(l) = m.get(p) {
                e.insert(l);
            }
        }
        for x in 0..self.w {
            g(self, &mut e, &Point{x, y: 0});
            g(self, &mut e, &Point{x, y: self.h-1});
        }
        for y in 0..self.h {
            g(self, &mut e, &Point{x: 0, y});
            g(self, &mut e, &Point{x: self.w-1, y});
        }
        e
    }

    /// Return the size of the largest area that isn't an escapee.
    pub fn largest(&self) -> usize {
        let esc = self.escapees();
        let mut cs = HashMap::<Landing, usize>::new();
        let mut best_count: usize = 0;
        for y in 0..self.h {
            for x in 0..self.w {
                if let Color::One(l) = self.get(&Point{x, y}) {
                    if !esc.contains(&l) {
                        let e = cs.entry(l).or_insert(0);
                        *e += 1;
                        if *e > best_count {
                            best_count = *e;
                        }
                    }
                }
            }
        }
        best_count
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                write!(f, "{}", match self.get(&Point{x, y}) {
                    Color::Unknown => '?',
                    Color::Many => '.',
                    Color::One(c) => {
                        (b'A' + c as u8) as char
                    },
                }).unwrap();
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let pts: Vec<_> = [(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)]
            .iter()
            .map(|(x, y)| Point{x: *x, y: *y})
            .collect();
        let m = Map::from_points(&pts);
        println!("{:?}", &m);

        let mut m1 = m.clone();
        loop {
            let m2 = m1.grow();
            if m2 == m1 { break };
            println!("{:?}", &m2);
            m1 = m2;
        }

        assert_eq!(m1, m.grow_completely());

        let mut hs: HashSet<Landing> = vec![0, 1, 2, 5].into_iter().collect(); // A, B, C, F
        assert_eq!(m1.escapees(), hs);

        assert_eq!(m1.largest(), 17);
    }
}