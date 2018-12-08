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
    let lines = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    let ns = parse_lines(lines);
    // println!("{:?}", ns);
    let sleepy_gid = most_sleepy(&ns);
    println!("Most sleepy guard: {}", sleepy_gid);
    let naps_for_gid: Vec<_> = ns.iter().filter(|s| s.guard == sleepy_gid).collect();
    let smin = sleepiest_minute(&naps_for_gid);
    println!("Sleepiest minute: {}", smin);
    println!("Product: {}", smin as u32 * sleepy_gid);
}

/// Return the guard who sleeps the most total minutes
fn most_sleepy(ns: &[Nap]) -> GuardID {
    // Make a total minutes slept per guard
    let mut mpg = BTreeMap::<u32, usize>::new();
    let mut sleepy_gid: Option<GuardID> = None;
    let mut sleepy_mins: usize = 0;
    for n in ns {
        let e = mpg.entry(n.guard).or_insert(0);
        *e += n.mins();
        if *e > sleepy_mins {
            sleepy_mins = *e;
            sleepy_gid = Some(n.guard);
        }
    }
    sleepy_gid.unwrap()
}

/// Return the minute (in 00:00 - 00:59) most commonly slept-for
fn sleepiest_minute(ns: &[&Nap]) -> usize {
    let mut sleeps = [0u32; 60];
    let mut max_i: Option<usize> = None;
    let mut max_n: u32 = 0;
    for n in ns {
        // Rust ranges are half-open, same as in these records: the guard is
        // awake in wake_min.
        for i in n.sleep_min..n.wake_min {
            sleeps[i] += 1;
            if sleeps[i] > max_n {
                max_i = Some(i);
                max_n = sleeps[i];
            }
        }
    }
    max_i.unwrap()
}

#[derive(Debug, Copy, Clone)]
struct Nap {
    guard: GuardID,
    sleep_min: usize,
    wake_min: usize,
}

impl Nap {
    fn mins(&self) -> usize {
        self.wake_min - self.sleep_min
    }
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
