/// https://adventofcode.com/2018/day/1#part2
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

/// Read a list of signed integers from stdin.
fn read_ints() -> Vec<i64> {
    let mut r = Vec::<i64>::new();
    for line in io::stdin().lock().lines() {
        r.push(line.unwrap().parse::<i64>().unwrap());
    }
    r
}

pub fn main() {
    let mut seen = HashSet::<i64>::new();
    let mut t = 0;
    for i in read_ints().iter().cycle() {
        seen.insert(t); // Visit 0 before incrementing
        t += i;
        println!("i={:<8} t={:<8}", i, t);
        if seen.contains(&t) {
            println!("first repeated value is {}", t);
            break;
        }
    }
}
