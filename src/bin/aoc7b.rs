/// https://adventofcode.com/2018/day/7
use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::io::prelude::*;

use lazy_static::lazy_static;
use regex::Regex;

pub fn main() {
    let cs = Constraints::from_strings(io::stdin().lock().lines().map(Result::unwrap));
    println!("total work: {:?}", cs.work(60, 5));
}

type Step = char;

#[derive(Debug, PartialEq)]
struct Constraints {
    /// From postcondition to set of preconditions
    deps: BTreeMap<Step, BTreeSet<Step>>,
}

type Time = u32;

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord)]
struct Worker {
    /// Time their current task will complete.
    completion: Time,
    /// Current task.
    task: Step,
}

fn step_time(s: Step) -> u32 {
    (s as u32) - ('A' as u32) + 1
}

impl Constraints {
    pub fn from_strings<S: AsRef<str>, I: Iterator<Item = S>>(s: I) -> Constraints {
        lazy_static! {
            static ref STEP_RE: Regex =
                Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin\.$")
                    .unwrap();
        }

        let mut deps = BTreeMap::new();
        for l in s {
            let cap = STEP_RE.captures(l.as_ref()).unwrap();
            let pre = cap.get(1).unwrap().as_str().chars().next().unwrap();
            let post = cap.get(2).unwrap().as_str().chars().next().unwrap();
            let e = deps.entry(post).or_insert_with(BTreeSet::new);
            assert!(e.insert(pre), "pair {:?} {:?} already present?", post, pre);
            // We also know the precondition exists
            deps.entry(pre).or_insert_with(BTreeSet::new);
        }

        Constraints { deps }
    }

    /// Return the next step available for anyone to do.
    pub fn next_step(&mut self) -> Option<Step> {
        if let Some(s) = self
            .deps
            .iter()
            .filter_map(|(k, v)| if v.is_empty() { Some(k) } else { None })
            .next()
        {
            Some(*s)
        } else {
            None
        }
    }

    /// Mark this step as completed; its dependencies are now available.
    pub fn complete(&mut self, s: Step) {
        for v in self.deps.values_mut() {
            v.remove(&s);
        }
    }

    /// Do all the work; remember how long it takes
    pub fn work(mut self, time_base: u32, n_workers: usize) -> u32 {
        let mut t = 0;
        // worker set is ordered by completion time
        let mut workers: Vec<Worker> = Vec::new();
        while !self.deps.is_empty() {
            while workers.len() < n_workers && !self.deps.is_empty() {
                println!("time {}", t);
                if let Some(task) = self.next_step() {
                    let w = Worker {
                        completion: t + step_time(task) + time_base,
                        task,
                    };
                    println!("start {:?}", w);
                    self.deps.remove(&task);
                    workers.push(w);
                } else {
                    println!("no tasks ready to start now");
                    break;
                }
            }
            // Next worker finishes. The vec manipulation is a bit gross, but
            // the vec is very small.
            let donew = workers.iter().min().unwrap().clone();
            workers.retain(|i| *i != donew);
            println!("complete {:?}", donew);
            t = donew.completion;
            self.complete(donew.task);
        }
        t
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let cs = Constraints::from_strings(
            "\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
                .split("\n"),
        );
        println!("{:?}", &cs);
        assert_eq!(cs.work(0, 2), 15);
    }
}
