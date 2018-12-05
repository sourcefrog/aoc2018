/// https://adventofcode.com/2018/day/1

use std::io;
use std::io::prelude::*;

pub fn main() {
    let stdin = io::stdin();
    let mut t = 0;
    for line in stdin.lock().lines() {
        let v = line.unwrap().parse::<i64>().unwrap();
        t += v;
    }
    println!("total: {}", t);
}