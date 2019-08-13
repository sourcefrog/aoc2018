//! https://adventofcode.com/2018/day/13
//!
//! Mine carts moving over tracks and sometimes colliding with each other.
//!
//! Part A is to find the location of the first collision.
//!
//! Part B is to find the location of the last remaining cart, after all
//! the others have collided.

// The ascii representation will do as a map, but we need to remember the cart
// locations and their per-cart intersection counters separately from the map,
// or we'll lose information about the map when the carts move over curves or
// intersections.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

/// Coordinates as (y, x).
type Coords = (usize, usize);

/// Returns the location of the first collision, and of the last remaining
/// cart.
fn solve() -> (Option<Coords>, Option<Coords>) {
    let mut s = String::with_capacity(8000);
    File::open("input/input13.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let mut m = Map::from_string(&s);
    m.play()
}

pub fn main() {
    let (first_coll, last_cart) = solve();
    println!("First collision at ({:?}", first_coll.unwrap());
    println!("Last remaining cart at {:?}", last_cart.unwrap());
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

#[derive(Clone, Eq, PartialEq)]
struct Map {
    /// Indexed by [y][x], a map of the track with no carts present.
    m: Vec<Vec<char>>,
    w: usize,
    h: usize,

    /// Indexed by the current position of the cart as [y][x], and the contents
    /// describe the state of the cart.  The index is in this order because the
    /// carts get to move in that order.
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
            m,
            carts,
            tick: 1,
        }
    }

    /// Take one step.
    ///
    /// Update this map. Return the Coords where the first collision of
    /// this step occurred, if any.
    pub fn step(&mut self) -> Option<Coords> {
        // First, collect all the positions: we'll visit carts in this
        // (y, x) order exactly once per tick, even as they move.
        //
        // For each cart, in this order, calculate a new position and direction.
        // The new position is one step from the current position, determined by
        // the cart's current direction, the track under it, and its turn counter.
        //
        // If there is already a cart there, crash. Otherwise, store this
        // cart there.
        let mut carts = self.carts.clone();
        let mut first_coll = None;

        let op: Vec<(usize, usize)> = carts.keys().cloned().collect();
        for p in op.iter() {
            let oldc = carts.remove(&p).unwrap();
            let newc = oldc.step(self);
            // println!("step {:?} to {:?}", oldc, newc);
            let newp = (newc.y, newc.x);
            if carts.contains_key(&newp) {
                println!("collision at {:?}", newp);
                first_coll = first_coll.or(Some(newp));
            }
            carts.insert(newp, newc);
        }
        self.carts = carts;
        self.tick += 1;
        first_coll
    }

    /// Play through to the conclusion.
    ///
    /// Returns optionally the location of the first collision (the solution
    /// to part A) and the location of the single last remaining cart if any
    /// (the solution to part B).
    pub fn play(&mut self) -> (Option<Coords>, Option<Coords>) {
        let mut first_coll = None;
        while self.carts.len() > 1 {
            first_coll = first_coll.or(self.step());
        }
        (first_coll, self.carts.keys().next().copied())
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
    fn correct_answers() {
        assert_eq!(solve().0, Some((22, 41)));
        // TODO: Check part B is Some((90, 84))));
    }

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

        let expect2 = &r"
/-->\        
|   |  /----\
| /-+--+-\  |
| | |  | |  |
\-+-/  \->--/
  \------/   
"[1..];
        assert_eq!(m.step(), None);
        check_map(&m, expect2);

        let expect3 = &r"
/---v        
|   |  /----\
| /-+--+-\  |
| | |  | |  |
\-+-/  \-+>-/
  \------/   
"[1..];
        assert_eq!(m.step(), None);
        check_map(&m, expect3);
        assert_eq!(m.tick, 3);

        let expect4 = &r"
/---\        
|   v  /----\
| /-+--+-\  |
| | |  | |  |
\-+-/  \-+->/
  \------/   
"[1..];
        assert_eq!(m.step(), None);
        check_map(&m, expect4);
        assert_eq!(m.tick, 4);

        let expect5 = &r"
/---\        
|   |  /----\
| /->--+-\  |
| | |  | |  |
\-+-/  \-+--^
  \------/   
"[1..];
        assert_eq!(m.step(), None);
        check_map(&m, expect5);
        assert_eq!(m.tick, 5);

        let expect14 = &r"
/---\        
|   |  /----\
| /-+--v-\  |
| | |  | |  |
\-+-/  ^-+--/
  \------/   
"[1..];
        for _i in 6..=14 {
            assert_eq!(m.step(), None);
        }
        check_map(&m, expect14);
        assert_eq!(m.tick, 14);

        assert_eq!(m.step(), Some((3, 7)));
    }

    fn check_map(m: &Map, expected: &str) {
        if m.render() != expected {
            panic!("unexpected map at tick {}:\n{}", m.tick, m.render());
        }
    }

    #[test]
    fn linear() {
        let mut m = Map::from_string(
            &"\
|
v
|
|
|
^
|
",
        );
        assert!(m.carts.contains_key(&(1, 0)));
        assert!(m.carts.contains_key(&(5, 0)));

        assert_eq!(m.step(), None);
        assert_eq!(m.step(), Some((3, 0)));
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
