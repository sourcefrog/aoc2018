// #![allow(unused)]

/// https://adventofcode.com/2018/day/20
use std::collections::BTreeSet;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

type Coord = i32;
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
struct Point {
    x: Coord,
    y: Coord,
}

impl Point {
    #[allow(unused)]
    pub fn update(&mut self, dir: Dir) {
        *self = self.step(dir);
    }

    pub fn step(self, dir: Dir) -> Point {
        let mut n = self.clone();
        match dir {
            Dir::N => n.y -= 1,
            Dir::S => n.y += 1,
            Dir::E => n.x += 1,
            Dir::W => n.x -= 1,
        };
        n
    }

    pub fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
}

#[allow(unused)]
fn pt(x: Coord, y: Coord) -> Point {
    Point { x, y }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W,
}

#[allow(unused)]
static DIRECTIONS: [Dir; 4] = [Dir::N, Dir::S, Dir::E, Dir::W];

impl Dir {
    fn from_char(c: char) -> Dir {
        match c {
            'N' => Dir::N,
            'S' => Dir::S,
            'E' => Dir::E,
            'W' => Dir::W,
            other => panic!("unexpected character {:?}", other),
        }
    }

    #[allow(unused)]
    fn to_char(self) -> char {
        match self {
            Dir::N => 'N',
            Dir::S => 'S',
            Dir::E => 'E',
            Dir::W => 'W',
        }
    }
}

/// Map of rooms that have been visited.
///
/// All we actually have to remember is which E-W and N-S doors we've passed through.
/// If we have gone through a door, then we know there must be rooms on either side
/// of it, and we can't know about rooms if we never pass through a door to or
/// from them.
///
/// Note that we can't pass directly between neighboring rooms unless there is a
/// door.
#[derive(Default, Debug)]
struct Map {
    /// For each entry, there's a door to the north.
    n_doors: BTreeSet<Point>,
    /// For each entry there's a door to the east.
    e_doors: BTreeSet<Point>,
}

impl Map {
    /// Note that you can move from p in direction d (and so also in the
    /// opposite direction.)
    fn record_move(&mut self, p: Point, d: Dir) {
        match d {
            Dir::N => self.n_doors.insert(p),
            Dir::E => self.e_doors.insert(p),
            Dir::S => self.n_doors.insert(p.step(d)),
            Dir::W => self.e_doors.insert(p.step(d)),
        };
    }
}

/// One of these is pushed every time we enter a new nested group, and popped
/// when finishing it. There's also one for the implicit top-level group.
#[derive(Debug)]
struct GroupState {
    /// The positions of active turtles at the point of entering this group.
    /// Each alternate branch will start from here.
    sps: BTreeSet<Point>,

    /// The total accumulated positions of active turtles at the end of
    /// evalutaing each branch. This doesn't include everything they might
    /// have moved through during the branch. But, if sub-branches multiply
    /// turtles, they'll all be here.
    eps: BTreeSet<Point>,
}

fn expand(r: &str) -> Map {
    // Walk through the string from left to right.
    //
    // When you see an open paren, push the map position onto a stack as being
    // where new siblings will start from, and push a new set of saved locations.
    //
    // On getting to a `|`, suspend this turtle (remembering its current location)
    // and start a new one at the map location of the start of this group.
    //
    // Also, keep a stack of sets of suspended turtles, that'll wake up at the
    // position we left them at.
    //
    // So when we see directions, we apply them in parallel to all the positions that were live at
    // the start of this group.
    let mut g = Vec::new();

    let mut map = Map::default();

    // Currently-live turtle positions, for the current branch.
    let mut turs = BTreeSet::new();
    turs.insert(Point::origin());

    for c in r.chars() {
        match c {
            'N' | 'E' | 'S' | 'W' => {
                let dir = Dir::from_char(c);
                let mut newturs = BTreeSet::new();
                for t in turs {
                    map.record_move(t, dir);
                    newturs.insert(t.step(dir));
                }
                turs = newturs;
            }
            '(' => {
                // Remember these starting positions, which will apply to
                // all groups inside.
                let gs = GroupState {
                    sps: turs.clone(),
                    eps: BTreeSet::new(),
                };
                // Hold onto these turtles and move them through the first
                // branch of the group.
                println!("Starting a new group {:?}", &gs);
                g.push(gs);
            }
            '|' => {
                // Remember these final points we reached, and resume them at
                // the end of this group. Then, create new turtles starting at
                // the beginning.
                let gs = g.last_mut().unwrap();
                gs.eps.extend(&turs);
                turs = gs.sps.clone();
                println!("Start branch of this group: {:?}", &gs);
            }
            ')' => {
                // All the final positions across all the branches, including
                // the currently active one, are the new current turtle positions.
                // Forget about the group and the start position.
                let gs = g.pop().unwrap();
                println!("Finish group: {:?}", &gs);
                turs.extend(&gs.eps);
                println!("After finishing group, turs={:?}", &turs);
            }
            _ => {
                panic!("unexpected char {:?}", c);
            }
        }
    }

    // At the end of the string there should be no more groups open.
    assert!(g.is_empty());
    println!("Final turtles: {:?}", &turs);
    println!("{} final points", turs.len());

    map
}

/// Return the canned input with newline, ^ and $ removed.
fn load_input() -> String {
    let mut s = String::with_capacity(100_000);
    File::open("input/input20.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    s.shrink_to_fit();
    assert!(s.ends_with("$\n"));
    assert!(s.starts_with('^'));
    s[1..(s.len() - 2)].to_string()
}

pub fn main() {
    let argv: Vec<_> = env::args().collect();
    if argv.len() > 1 {
        expand(&argv[1]);
    } else {
        let inp = load_input();
        expand(&inp);
    }
}

#[cfg(test)]
mod test {
    use super::pt;
    use super::Point;

    #[test]
    fn test_load_input() {
        super::load_input();
    }

    #[test]
    fn test_expand_input() {
        let inst = super::load_input();
        println!("{:?} bytes of input", inst.len());
    }

    #[test]
    fn example1() {
        // Example given in the problem statement:
        //
        // #########
        // #.|.|.|.#
        // #-#######
        // #.|.|.|.#
        // #-#####-#
        // #.#.#X|.#
        // #-#-#####
        // #.|.|.|.#
        // #########
        //
        // They're all sorted by x first then y, and y runs downwards. X is the origin in the
        // diagram.
        let map = super::expand("ENWWW(NEEE|SSE(EE|N))");
        let e_doors: Vec<Point> = map.e_doors.into_iter().collect();
        assert_eq!(
            e_doors,
            vec![
                pt(-2, -2),
                pt(-2, -1),
                pt(-2, 1),
                pt(-1, -2),
                pt(-1, -1),
                pt(-1, 1),
                pt(0, -2),
                pt(0, -1),
                pt(0, 0),
                pt(0, 1)
            ]
        );
        let n_doors: Vec<Point> = map.n_doors.into_iter().collect();
        assert_eq!(
            n_doors,
            vec![pt(-2, -1), pt(-2, 0), pt(-2, 1), pt(-1, 1), pt(1, 0),]
        );
    }
}
