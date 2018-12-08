/// https://adventofcode.com/2018/day/4
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::BTreeMap;
use std::io;
use std::io::prelude::*;
use std::iter::Iterator;

use regex::Regex;

type GuardID = u32;

pub fn main() {
    let ns = parse_lines(
        io::stdin()
            .lock()
            .lines()
            .map(Result::unwrap)
            .collect::<Vec<_>>(),
    );
    let mut min_by_guard = BTreeMap::<GuardID, [u32; 60]>::new();
    let mut best_guard: Option<GuardID> = None;
    let mut best_sleeps = 0;
    let mut best_i: Option<usize> = None;
    for n in ns {
        let sg = min_by_guard.entry(n.guard).or_insert([0u32; 60]);
        for i in n.sleep_min..n.wake_min {
            sg[i] += 1;
            if sg[i] > best_sleeps {
                best_sleeps = sg[i];
                best_i = Some(i);
                best_guard = Some(n.guard);
            }
        }
    }
    let best_guard = best_guard.unwrap();
    let best_i = best_i.unwrap() as u32;
    println!(
        "Most sleepy: guard {} on minute: {}. Product = {}",
        best_guard,
        best_i,
        best_guard * best_i,
    );
}

#[derive(Debug, Copy, Clone)]
struct Nap {
    guard: GuardID,
    sleep_min: usize,
    wake_min: usize,
}

fn ci(caps: &regex::Captures, i: usize) -> u32 {
    caps.get(i).unwrap().as_str().parse().unwrap()
}

/// Parse a slice of lines into a vec of stints.
fn parse_lines(mut ls: Vec<String>) -> Vec<Nap> {
    lazy_static! {
        static ref GUARD_RE: Regex =
            Regex::new(r"\[....-..-.. ..:..\] Guard #(\d+) begins shift").unwrap();
        static ref WAKE_RE: Regex = Regex::new(r"\[....-..-.. 00:(..)\] wakes up$").unwrap();
        static ref SLEEP_RE: Regex = Regex::new(r"\[....-..-.. 00:(..)\] falls asleep$").unwrap();
    }
    ls.sort();
    let mut ns = Vec::new();
    let mut guard: Option<GuardID> = None;
    let mut sleep_min: Option<usize> = None;
    for l in ls {
        println!("{}", l);
        if let Some(c) = GUARD_RE.captures(&l) {
            guard = Some(ci(&c, 1));
            println!("Guard {}", guard.unwrap());
        } else if let Some(c) = SLEEP_RE.captures(&l) {
            assert!(guard.is_some());
            assert!(sleep_min.is_none());
            sleep_min = Some(ci(&c, 1) as usize);
        } else if let Some(c) = WAKE_RE.captures(&l) {
            assert!(guard.is_some());
            assert!(sleep_min.is_some());
            ns.push(Nap {
                guard: guard.unwrap(),
                sleep_min: sleep_min.take().unwrap(),
                wake_min: ci(&c, 1) as usize,
            });
        } else {
            panic!("unrecognized line {:?}", l);
        }
    }
    ns
}
