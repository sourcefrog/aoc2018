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

#![allow(dead_code)]

use aoc2018::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Thing {
    Open,
    Trees,
    Lumberyard,
}
use self::Thing::*;

impl Thing {
    pub fn from_char(c: char) -> Thing {
        match c {
            '.' => Open,
            '|' => Trees,
            '#' => Lumberyard,
            other => panic!("unexpected character {:?}", other),
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Open => '.',
            Trees => '|',
            Lumberyard => '#',
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Map {
    m: Matrix<Thing>,
}

impl Map {
    pub fn parse(s: &str) -> Map {
        let mut mb = Matrix::from_rows();
        for l in s.lines() {
            let v: Vec<Thing> = l.chars().map(Thing::from_char).collect();
            mb.add_row(&v);
        }
        Map { m: mb.finish() }
    }

    pub fn render(&self) -> String {
        let mut s = String::with_capacity(self.m.height() * (self.m.width() + 1));
        for y in 0..self.m.height() {
            for x in 0..self.m.width() {
                s.push(self.m[point(x, y)].to_char())
            }
            s.push('\n')
        }
        s
    }

    pub fn step(&self) -> Map {
        let mut newm = Matrix::new(self.m.width(), self.m.height(), Open);
        for p in self.m.iter_points() {
            let ns = self.m.neighbor8_values(p);
            newm[p] = match self.m[p] {
                Open => {
                    if ns.into_iter().filter(|n| *n == Trees).count() >= 3 {
                        Trees
                    } else {
                        Open
                    }
                }
                Trees => {
                    if ns.into_iter().filter(|n| *n == Lumberyard).count() >= 3 {
                        Lumberyard
                    } else {
                        Trees
                    }
                }
                Lumberyard => {
                    if ns.contains(&Lumberyard) && ns.contains(&Trees) {
                        Lumberyard
                    } else {
                        Open
                    }
                }
            }
        }
        Map { m: newm }
    }

    pub fn count(&self, th: Thing) -> usize {
        self.m.values().filter(|t| **t == th).count()
    }

    pub fn resource_value(&self) -> usize {
        self.count(Trees) * self.count(Lumberyard)
    }
}

fn load_input() -> Map {
    use std::fs::File;
    use std::io::Read;
    let mut s = String::new();
    File::open("input/input18.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    Map::parse(&s)
}

fn solve18a() -> usize {
    let mut m = load_input();
    for _i in 0..10 {
        m = m.step();
    }
    m.resource_value()
}

fn solve18b() -> usize {
    let mut m = load_input();
    const N: u64 = 1_000_000_000;
    // Eventually, the map reaches a cycle. Since state n completely
    // determines state n+1, if we find a cycle of any length,
    // we know that cycle will repeat ad infinitum.
    // 1000 is chosen by trial and error to be enough for it to
    // enter a cycle.
    const STAB: u64 = 1000;
    for _i in 0..STAB {
        // Run a bit to let it stabilize
        m = m.step();
    }
    let mstab = m.clone();
    let mut j = STAB;
    let cycle = loop {
        m = m.step();
        // println!("{}\n{}", j, m.render());
        println!("{}", j);
        if m == mstab {
            println!("Found cycle: gen {} == gen {}", j, STAB);
            break (j - STAB) as u64;
        }
        j += 1;
    };
    // How many more times to advance, to get aligned with N?
    let more = (N - j) % cycle;
    println!(
        "{} from {} is {} full cycles and {} extra steps",
        N,
        j,
        (N - j) / cycle,
        more
    );
    for _ in 0..more {
        m.step();
        j += 1;
    }
    m.resource_value()
}

pub fn main() {
    // println!("resource value after 10 = {}", solve18a());
    println!("after 1 billion = {}", solve18b());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let m = Map::parse(
            "\
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
",
        );
        println!("{}", m.render());
        let mut m = m.step();
        println!("{}", m.render());

        for _i in 2..=10 {
            m = m.step();
            println!("{}", m.render());
        }
        println!("resource value = {}", m.resource_value());
    }

    #[test]
    fn solution18a() {
        assert_eq!(solve18a(), 511000);
    }

    #[test]
    fn solution18b() {
        // known correct solution to 18b
        assert_eq!(solve18b(), 194934);
    }
}
