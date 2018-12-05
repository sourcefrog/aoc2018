/// https://adventofcode.com/2018/day/1

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
    let t: i64 = read_ints().iter().sum();
    println!("total: {}", t);
}