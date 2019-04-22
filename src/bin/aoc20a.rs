#![allow(unused)]

/// https://adventofcode.com/2018/day/20
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

type Coord = i32;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Point {
    x: Coord,
    y: Coord,
}

type StateId = usize;
const NONE: StateId = 0;
const ORIGIN: StateId = 1;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W,
}

static DIRECTIONS: [Dir; 4] = [Dir::N, Dir::S, Dir::E, Dir::W];

impl Dir {
    fn from_char(c: char) -> Dir {
        match c {
            'N' => Dir::N,
            'S' => Dir::S,
            'E' => Dir::E,
            'W' => Dir::W,
            other => panic!(other),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Dir::N => 'N',
            Dir::S => 'S',
            Dir::E => 'E',
            Dir::W => 'W',
        }
    }
}

#[derive(Debug)]
struct StateMap {
    // State machine, indexed by arbitrary state numbers, with each one having
    // up to four links outward, depending on the direction. State 0 is unused,
    // 1 is the origin.
    m: Vec<[StateId; 4]>,
}

impl StateMap {
    /// Add a link saying that if we're in `from_state` and we go `direction` we get to a new
    /// state, or return an existing such link.
    fn get_or_insert(&mut self, from_state: StateId, direction: Dir) -> StateId {
        assert!(from_state > 0);
        let dir_i = direction as usize;
        let existing = self.m[from_state][dir_i];
        if existing > 0 {
            existing
        } else {
            let new_state = self.m.len();
            self.m.push([NONE; 4]);
            self.m.get_mut(from_state).unwrap()[dir_i] = new_state;
            new_state
        }
    }

    /// Build a state map from a regexp.
    fn from_str(r: &str) -> StateMap {
        debug_assert!(r.is_ascii());
        let mut new = StateMap {
            m: vec![[NONE; 4], [NONE; 4], [NONE; 4]],
        };
        let mut last = ORIGIN;
        for c in r.chars() {
            last = new.get_or_insert(last, Dir::from_char(c));
        }
        new
    }

    /// Return true if the given string matches this map.
    fn is_partial_match(&self, a: &str) -> bool {
        let mut s: StateId = ORIGIN;
        for c in a.chars() {
            let next = self.m[s][Dir::from_char(c) as usize];
            println!("{:?} => state {:?}", c, next);
            if next == NONE {
                // c isn't accepted here.
                return false;
            }
            s = next;
        }
        // Note, we can't currently determine if this string completely satisfies the regexp or
        // ends early, because we don't encode an 'end' move. It doesn't seem important for these
        // purposes, yet.
        true
    }

    /// Yield all possible matches for this regexp.
    fn all_matches(&self) -> Vec<String> {
        // States and prefixes that we still need to consider.
        let mut queue: Vec<(String, StateId)> = vec![(String::new(), ORIGIN)];
        let mut r = Vec::new();
        while let Some((prefix, c)) = queue.pop() {
            if self.m[c] == [NONE; 4] {
                // No more steps from here, so we reached the end of a string.
                println!("found {:?}", prefix);
                r.push(prefix);
            } else {
                // Descend through all possible children.
                for dir in DIRECTIONS.iter() {
                    let d = *dir as usize;
                    let newstate = self.m[c][d];
                    if newstate != NONE {
                        let mut newp = prefix.clone();
                        newp.push(dir.to_char());
                        println!("move from {} in {:?} to {}", c, newp, newstate);
                        queue.push((newp, newstate));
                    }
                }
            }
        }
        r
    }
}

/// Parse out the first alternate group from a string, returning the
/// head (before the alternate), a vec of alternatives, and then the
/// tail after it. Only the topmost alternative is parsed-out, so any of
/// the alts might themselves contain alternatives that later need to be expanded.
fn parse_alternate(s: &[u8]) -> (&[u8], Vec<&[u8]>, &[u8]) {
    let mut i = 0;
    let mut head = &s[..0];
    // Pull off any head section before the parens
    loop {
        if i >= s.len() {
            return (s, vec![], &s[i..]); // contains no paren
        } else if s[i] == b'(' {
            head = &s[..i];
            break;
        }
        i += 1;
    }
    // Skip over opening paren
    debug_assert_eq!(s[i], b'(');
    i += 1;

    // Walk over alternates, accumulating each one into a vector, breaking
    // on top-level pipes or the closing bracket.
    let mut alts = Vec::new();
    let mut level = 0;
    let mut ai = i;
    loop {
        let c = s[i];
        if level == 0 {
            match c {
                b'(' => {
                    level += 1;
                }
                b'|' => {
                    alts.push(&s[ai..i]);
                    ai = i + 1;
                }
                b')' => {
                    // conclude the last alternate, then stop
                    alts.push(&s[ai..i]);
                    i += 1;
                    break;
                }
                _other => (),
            }
        } else {
            // not at the top level, so just count parens for the sake of grouping, and skip
            // everything else.
            level += match c {
                b'(' => 1,
                b')' => -1,
                _other => 0,
            }
        }
        i += 1;
    }
    let tail = &s[i..];
    (head, alts, tail)
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
    assert!(s.starts_with("^"));
    s[1..(s.len() - 2)].to_string()
}

pub fn main() {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::parse_alternate;

    fn check_parse(s: &str, head: &str, alts: Vec<&str>, tail: &str) {
        let (h, a, t) = parse_alternate(s.as_bytes());
        assert_eq!(h, head.as_bytes());
        assert_eq!(t, tail.as_bytes());
        assert_eq!(a.len(), alts.len());
        for (ia, ib) in a.iter().zip(alts.iter()) {
            assert_eq!(ia, &ib.as_bytes());
        }
    }

    #[test]
    fn test_parse_alternate() {
        check_parse("", "", vec![], "");
        check_parse("ABC", "ABC", vec![], "");
        check_parse("ABC()()", "ABC", vec![""], "()");
        check_parse("ABC()", "ABC", vec![""], "");
        check_parse("((N|S)|(E|W))", "", vec!["(N|S)", "(E|W)"], "");
        check_parse("NEWS((N|E|)EE|WW)", "NEWS", vec!["(N|E|)EE", "WW"], "");
        check_parse("NEWS(N|E|)EE", "NEWS", vec!["N", "E", ""], "EE");
    }

    #[test]
    fn test_statemap_from_str() {
        use super::StateMap;
        let m = StateMap::from_str("NEWS");
        println!("{:#?}", m);
        assert!(m.is_partial_match("NEWS"));
        assert!(m.is_partial_match("NE"));
        assert!(!m.is_partial_match("NN"));
        assert!(!m.is_partial_match("EWS"));
        assert_eq!(m.all_matches(), vec!["NEWS".to_string()]);
    }

    #[test]
    fn test_load_input() {
        super::load_input();
    }

    #[test]
    fn test_expand_input() {
        let inst = super::load_input();
        println!("{:?} bytes of input", inst.len());
    }
}
