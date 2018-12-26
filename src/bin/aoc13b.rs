/// https://adventofcode.com/2018/day/13
// The ascii representation will do as a map, but we need to remember
// the cart locations and their per-cart intersection counters separately from the map,
// or we'll lose information about the map when the carts move over curves or intersections.
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use either::Either;

pub fn main() {
    let mut s = String::with_capacity(8000);
    File::open("input/input13.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let mut m = Map::from_string(&s);
    loop {
        match m.step() {
            Either::Left(newm) => {
                m = newm;
            }
            Either::Right(p) => {
                println!("Last remaining cart is at x={}, y={}", p.1, p.0);
                break;
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_char(c: char) -> Direction {
        match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '^' => Direction::Up,
            e => panic!("unknown direction {:?}", e),
        }
    }

    #[allow(dead_code)]
    pub fn to_char(self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }

    pub fn plain_track(self) -> char {
        match self {
            Direction::Up | Direction::Down => '|',
            Direction::Left | Direction::Right => '-',
        }
    }

    pub fn delta(self, p: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (p.0.checked_sub(1).unwrap(), p.1),
            Direction::Down => (p.0.checked_add(1).unwrap(), p.1),
            Direction::Left => (p.0, p.1.checked_sub(1).unwrap()),
            Direction::Right => (p.0, p.1.checked_add(1).unwrap()),
        }
    }

    pub fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    pub fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Cart {
    /// Number of intersections this has passed, starting at 0.
    inters: usize,

    /// Current direction,
    dir: Direction,

    x: usize,
    y: usize,
}

impl Cart {
    pub fn new(dir: Direction, pos: (usize, usize)) -> Cart {
        Cart {
            dir,
            inters: 0,
            y: pos.0,
            x: pos.1,
        }
    }

    /// Calculate the new position and state
    pub fn step(&self, m: &Map) -> Cart {
        // First, move, governed only by the current direction.
        let (y, x) = self.dir.delta((self.y, self.x));

        // Now, maybe turn, depending on what's at the new point, and our entry
        // direction, and our turn counter.
        let c = m.look((y, x));
        let dir = match c {
            '-' | '|' => self.dir,
            '+' => match self.inters {
                0 => self.dir.turn_left(),
                1 => self.dir,
                2 => self.dir.turn_right(),
                other => panic!("bad inters {:?}", other),
            },
            '\\' => match self.dir {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
            '/' => match self.dir {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            other => panic!("unimplemented map character {:?}", other),
        };

        // Increment intersection counter if passing through an intersection
        let inters = if c == '+' {
            (self.inters + 1) % 3
        } else {
            self.inters
        };

        Cart { dir, inters, y, x }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Map {
    /// Indexed by [y][x]
    m: Rc<Vec<Vec<char>>>,
    w: usize,
    h: usize,

    /// Indexed by [y][x], and the value is the number of turns that cart has made,
    /// starting at 0, .
    carts: BTreeMap<(usize, usize), Cart>,

    tick: usize,
}

impl Map {
    pub fn from_string(s: &str) -> Map {
        let mut m: Vec<Vec<char>> = Vec::new();
        let mut carts = BTreeMap::new();
        for (y, l) in s.lines().enumerate() {
            let mut row = Vec::new();
            let lw = l.len();
            if !m.is_empty() {
                assert_eq!(lw, m[0].len(), "{:?} different length from {:?}", l, m[0]);
            }
            for (x, c) in l.chars().enumerate() {
                // If there's a cart, remember that location and also strip it out.
                let c = match c {
                    '<' | '>' | 'v' | '^' => {
                        let dir = Direction::from_char(c);
                        let pos = (y, x);
                        carts.insert(pos, Cart::new(dir, pos));
                        dir.plain_track()
                    }
                    _ => c,
                };
                row.push(c);
            }
            m.push(row)
        }
        Map {
            w: m[0].len(),
            h: m.len(),
            m: Rc::new(m),
            carts,
            tick: 1,
        }
    }

    /// Return the new map, or the location of the one removing cart, if there is only one left.
    pub fn step(&self) -> Either<Map, (usize, usize)> {
        // First, collect all the positions: we'll visit carts in this
        // (y, x) order exactly once per tick, even as they move.
        //
        // For each cart, in this order, calculate a new position and direction.
        // The new position is one step from the current position, determined by
        // the cart's current direction, the track under it, and its turn counter.
        //
        // If there is already a cart there, crash. Otherwise, store this cart there.
        println!("** Tick {}, {} carts remain", self.tick, self.carts.len());
        let mut carts = self.carts.clone();
        let op: Vec<(usize, usize)> = carts.keys().cloned().collect();
        for p in op.iter() {
            if let Some(oldc) = carts.remove(&p) {
                let newc = oldc.step(self);
                // println!("step {:?} to {:?}", oldc, newc);
                let newp = (newc.y, newc.x);
                if carts.contains_key(&newp) {
                    println!("collision at {:?}", newp);
                    carts.remove(&newp);
                } else {
                    carts.insert(newp, newc);
                }
            } else {
                // Eliminated by a collision earlier in this round
            }
        }
        if carts.len() == 1 {
            Either::Right(
                *carts.keys().next().unwrap()
            )
        } else if carts.is_empty() {
            panic!("no more carts");
        } else {
            Either::Left(Map {
                w: self.w,
                h: self.h,
                m: self.m.clone(),
                carts,
                tick: self.tick + 1,
            })
        }
    }

    #[cfg(test)]
    pub fn render(&self) -> String {
        let mut s = String::with_capacity(self.h * (self.w + 1));
        for y in 0..self.h {
            for x in 0..self.w {
                if let Some(cart) = self.carts.get(&(y, x)) {
                    s.push(cart.dir.to_char())
                } else {
                    s.push(self.m[y][x])
                }
            }
            s.push('\n');
        }
        s
    }

    /// Get the underlying map character
    pub fn look(&self, p: (usize, usize)) -> char {
        self.m[p.0][p.1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let mapstr1 = &r"
/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   
"[1..];
        let mut m = Map::from_string(mapstr1);
        assert_eq!(m.w, 13);
        assert_eq!(m.h, 6);
        assert_eq!(m.tick, 1);
        assert_eq!(m.render(), mapstr1);

        for _ in 0..100 {
            match m.step() {
                Either::Left(newm) => {
                    m = newm;
                },
                Either::Right(p) => {
                    assert_eq!(p, (4, 6));
                },
            }
        }
        unreachable!();
    }

    #[allow(dead_code)]
    fn check_map(m: &Map, expected: &str) {
        if m.render() != expected {
            panic!("unexpected map at tick {}:\n{}", m.tick, m.render());
        }
    }

    #[test]
    fn cart_step() {
        let m = Map::from_string(
            "\
-----
",
        );
        let c = Cart::new(Direction::Right, (0, 0));
        let nc = c.step(&m);
        assert_eq!(
            nc,
            Cart {
                dir: Direction::Right,
                y: 0,
                x: 1,
                inters: 0,
            }
        );
    }

    #[test]
    fn cart_step_down() {
        let m = Map::from_string(
            "\
|
|
|
|
",
        );
        let c = Cart::new(Direction::Down, (0, 0));
        assert_eq!(
            c.step(&m),
            Cart {
                dir: Direction::Down,
                y: 1,
                x: 0,
                inters: 0,
            }
        );
    }
}
