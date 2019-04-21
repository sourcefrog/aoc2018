#![allow(unused)]

use std::collections::VecDeque;

/// https://adventofcode.com/2018/day/20

type Coord = i32;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Point {
    x: Coord,
    y: Coord,
}


#[derive(Debug)]
struct Turtle {
    /// Current location
    p: Point,

    /// Index into the instruction vector.
    idx: usize,

    /// The positions we were at when we saw the last open parens, and from which new siblings will
    /// spawn.
    open_p: Vec<Point>,

    /// The paths we were at when we saw the last open parens.
    open_path: Vec<String>,

    /// Identifier number for this turtle to make them easier to track.
    turtle_id: usize,

    /// Path, for debugging.
    path: String,
}


fn walk(r: &str) {
    assert!(r.starts_with("^"));
    assert!(r.ends_with("$"));
    assert!(r.is_ascii());
    let r = &r[1..(r.len()-1)];
    println!("r inner = {:?}", r);
    let cs: Vec<char> = r.chars().collect();
    run_top(&cs);
}


/// Return the index of the next unmatched closing paren after `i`, which must
/// be inside an open paren.
fn skip_to_close(i: usize, r: &[char]) -> usize {
    let mut i = i;
    let mut level = 0;
    loop {
        let c = r[i];
        if c == ')' {
            if level == 0 {
                return i;
            } else {
                level -= 1;
            }
        } else if c == '(' {
            level += 1;
        }
        i += 1;
    }
}


fn run_top(r: &[char]) {
    let mut saved: VecDeque<Turtle> = VecDeque::new();
    let mut turtle_id = 0;
    saved.push_back(Turtle {
        p: Point { x: 0, y: 0 },
        idx: 0,
        open_p: Vec::new(),
        open_path: Vec::new(),
        turtle_id,
        path: String::new(),
    });
    while let Some(mut t) = saved.pop_front() {
        println!("** moving turtle {:?}, starting at {:?}", t.turtle_id, t.p);
        let mut idx = t.idx;
        while idx < r.len() {
            let c = r[idx];
            match c {
                'N' | 'S' | 'E' | 'W' => {
                    // Step in that direction, advance to next character.
                    // TODO: Actually update the location.
                    println!("{:?}", c);
                    t.path.push(c);
                },
                '(' => {
                    // Remember this location, so that we can later spawn from here.  Then,
                    // continue this turtle walking down the first branch.
                    t.open_p.push(t.p);
                    t.open_path.push(t.path.clone());
                },
                '|' => {
                    // But for this turtle, just skip to the end point of the last-saved
                    // location.
                    println!("looking at pipe at {:?}...", idx);
                    let last_p = t.open_p.last().unwrap();
                    turtle_id += 1;
                    let newt = Turtle {
                        p: *last_p,
                        idx: idx + 1,
                        open_p: t.open_p.clone(),
                        open_path: t.open_path.clone(),
                        turtle_id,
                        path: t.open_path.last().unwrap().clone(),
                    };
                    println!("spawn new turtle {:?}", newt);
                    // TODO: Avoid spawning exact-duplicate turtles for everyone that proceeds
                    // past a branch. Just one is enough.
                    saved.push_back(newt);
                    idx = skip_to_close(idx, r);
                    println!("skip to close paren at {:?}", idx);
                    continue;
                },
                ')' => {
                    // Whether we jumped here from a pipe, or reached here through natural
                    // exhaustion, we're done. Forget about the last-opened paren and position,
                    // and proceed onwards.
                    println!("reached close paren at {}", idx);
                    t.open_p.pop().unwrap();
                    t.open_path.pop().unwrap();
                },
                other => panic!("unexpected instruction {:?}", other),
            }
            idx += 1;
        }
        println!("turtle {:?} completed, path was {:?}", t.turtle_id, t.path);
    }
}


pub fn main() {
    //walk("^WNE$");
    // walk("^WNE(EEE(SS|NN)|WWW|)$");
    // walk("^WNE(EEE(SS|NN)|WWW)$");
    walk("^W((EE|SS)|(NN|WW))$");
    // walk("^WNE(EEE|WWW|)$");
}
