// Copyright 2018 Google LLC
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     https://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! https://adventofcode.com/2018/day/7
//!
//! The input is small enough we can use a somewhat brute-force algorithm:
//!
//! Parse the input into a sorted map from not-yet-executed postconditions,
//! to a set of as-yet-unmet preconditions.
//!
//! Walk that map to find the alphabetically first step that has no unmet
//! preconditions. Emit it, and then go through the map and remove it from
//! all precondition sets.

use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::io::prelude::*;

use lazy_static::lazy_static;
use regex::Regex;

pub fn main() {
    let cs = Constraints::from_strings(io::stdin().lock().lines().map(Result::unwrap));
    println!("{:?}", cs);
    println!("order: {:?}", cs.find_order());
}

type Step = char;

#[derive(Debug, PartialEq)]
struct Constraints {
    /// From postcondition to set of preconditions
    deps: BTreeMap<Step, BTreeSet<Step>>,
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
            if let Some(cap) = STEP_RE.captures(l.as_ref()) {
                let pre = cap.get(1).unwrap().as_str().chars().next().unwrap();
                let post = cap.get(2).unwrap().as_str().chars().next().unwrap();
                let e = deps.entry(post).or_insert_with(BTreeSet::new);
                assert!(e.insert(pre), "pair {:?} {:?} already present?", post, pre);
                // We also know the precondition exists
                deps.entry(pre).or_insert_with(BTreeSet::new);
            } else {
                panic!("Can't parse {:?}", l.as_ref());
            }
        }

        Constraints { deps }
    }

    pub fn find_order(&self) -> String {
        let mut s = String::new();
        let mut deps = self.deps.clone();
        while !deps.is_empty() {
            let next = *deps
                .iter()
                .filter_map(|(k, v)| if v.is_empty() { Some(k) } else { None })
                .next()
                .unwrap();
            deps.remove(&next);
            for v in deps.values_mut() {
                v.remove(&next);
            }
            s.push(next);
        }
        s
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
        let order = cs.find_order();
        assert_eq!(order, "CABDFE");
    }
}
