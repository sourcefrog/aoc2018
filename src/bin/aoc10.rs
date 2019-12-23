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

//! https://adventofcode.com/2018/day/10
use std::io;
use std::io::prelude::*;

use lazy_static::lazy_static;
use regex::Regex;

pub fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut map = Map::parse(&s);
    // Wait for it to get small enough
    while !map.draw() {
        map.step();
    }
    // Keep drawing while it stays small
    while map.draw() {
        map.step();
    }
}

#[derive(Debug)]
pub struct Star {
    pos: (i32, i32),
    vel: (i32, i32),
}

fn ci(caps: &regex::Captures, i: usize) -> i32 {
    caps.get(i).unwrap().as_str().parse().unwrap()
}

#[derive(Debug)]
struct Map {
    ss: Vec<Star>,
    steps: usize,
}

impl Map {
    pub fn parse(r: &str) -> Map {
        lazy_static! {
            static ref STAR_RE: Regex = Regex::new(
                r"^position=< *([0-9-]+), *([0-9-]+)> velocity=< *([0-9-]+), *([0-9-]+)>$"
            )
            .unwrap();
        }
        let mut ss = Vec::new();
        for l in r.split('\n') {
            if l.is_empty() {
            } else if let Some(caps) = STAR_RE.captures(l) {
                ss.push(Star {
                    pos: (ci(&caps, 1), ci(&caps, 2)),
                    vel: (ci(&caps, 3), ci(&caps, 4)),
                });
            } else {
                panic!("Can't parse {:?}", l);
            }
        }
        Map { ss, steps: 0 }
    }

    // Returns true if it's feasible to draw.
    pub fn draw(&self) -> bool {
        let ss = &self.ss;
        let x_min = ss.iter().map(|s| s.pos.0).min().unwrap();
        let x_max = ss.iter().map(|s| s.pos.0).max().unwrap();
        let w = x_max - x_min;
        let y_min = ss.iter().map(|s| s.pos.1).min().unwrap();
        let y_max = ss.iter().map(|s| s.pos.1).max().unwrap();
        let h = y_max - y_min;
        // println!(
        //     "x={}..={} ({}), y={}..={} ({}), n={}",
        //     x_min,
        //     x_max,
        //     w,
        //     y_min,
        //     y_max,
        //     h,
        //     ss.len()
        // );
        if w > 80 || h > 60 {
            // println!("too big!");
            return false;
        }
        println!("step {}", self.steps);
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                print!("{}", if self.lit((x, y)) { '#' } else { '.' });
            }
            println!();
        }
        true
    }

    fn lit(&self, pos: (i32, i32)) -> bool {
        self.ss.iter().any(|s| s.pos == pos)
    }

    pub fn step(&mut self) {
        for s in self.ss.iter_mut() {
            s.pos.0 += s.vel.0;
            s.pos.1 += s.vel.1;
        }
        self.steps += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static HI_DEF: &str = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";

    #[test]
    fn hi() {
        let mut map = Map::parse(&HI_DEF);
        map.draw();

        for _i in 0..3 {
            map.step();
            map.draw();
        }
    }
}
