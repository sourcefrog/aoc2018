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

//! https://adventofcode.com/2018/day/9
use std::fmt;
use std::fmt::{Debug, Formatter};

pub fn main() {
    // 432 players; last marble is worth 71019 points
    println!(
        "High score {}",
        Circle::new(432).play_until(71019).high_score()
    );

    // Part B
    println!(
        "High score {}",
        Circle::new(432).play_until(71019 * 100).high_score()
    );
}

/// The marbles in a circle, each represented by its unique number, starting
/// at 0.
struct Circle {
    /// For marbles in the circle, we remember the backward and forward links,
    /// to represent effectively a circular doubly-linked list by marble number.
    ///
    /// Marbles may also be removed from the list, at which time their backward
    /// and forward links are both set to usize::MAX;
    ///
    /// links[i] describes the links from marble i.
    links: Vec<(usize, usize)>,

    /// There's also a concept of the current marble, which must be included
    /// in the circle.
    current: usize,

    /// An entry for each player
    scores: Vec<usize>,

    /// Next player to go; 0-based.
    player: usize,
}

const UNLINKED: (usize, usize) = (usize::max_value(), usize::max_value());

impl Circle {
    pub fn new(n_players: usize) -> Circle {
        Circle {
            links: vec![(0, 0)],
            current: 0,
            scores: vec![0; n_players],
            player: 0,
        }
    }

    pub fn next_marble(&self) -> usize {
        self.links.len()
    }

    pub fn succ(&self, i: usize) -> usize {
        assert_eq!(self.links[self.links[i].1].0, i);
        self.links[i].1
    }

    pub fn pred(&self, i: usize) -> usize {
        assert_ne!(i, usize::max_value());
        assert_eq!(self.links[self.links[i].0].1, i);
        self.links[i].0
    }

    /// Push the next marble into the circle after i
    pub fn insert_next(&mut self) -> usize {
        let i = self.succ(self.current);
        let n = self.next_marble();
        self.links.push((i, self.succ(i)));
        let follow = self.succ(i) as usize;
        self.links[follow].0 = n;
        self.links[i as usize].1 = n;
        self.current = n;
        n
    }

    pub fn remove_7back(&mut self) -> usize {
        let mut c = self.current;
        for _i in 0..7 {
            c = self.pred(c);
        }
        let (prev, next) = self.links[c];
        // println!("unlink {} - {} - {}", prev, c, next);
        assert!(prev != c && next != c && next != prev);
        self.links[prev].1 = next;
        self.links[next].0 = prev;
        self.current = next;
        self.links[c] = UNLINKED;
        c
    }

    /// Insert the next marble or do the special thing for modulo 23.
    pub fn step(&mut self) {
        let n = self.next_marble();
        if n % 23 == 0 {
            self.scores[self.player] += n + self.remove_7back();
            // Mark this marble as played, but not in the circle
            self.links.push(UNLINKED);
        } else {
            // println!("insert {}", n);
            self.insert_next();
        }
        self.player = (self.player + 1) % self.scores.len();
    }

    pub fn play_until(mut self, marble: usize) -> Circle {
        while self.next_marble() <= marble {
            self.step();
        }
        self
    }

    /// The numerically lowest present marble
    pub fn top(&self) -> usize {
        for i in 0..self.links.len() {
            if self.links[i] != UNLINKED {
                return i;
            }
        }
        panic!("Circle empty?");
    }

    pub fn high_score(&self) -> usize {
        *self.scores.iter().max().unwrap()
    }
}

impl Debug for Circle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let top = self.top();
        let mut i = top;
        loop {
            let ll = self.links[i as usize];
            assert!(ll != UNLINKED);
            if i == self.current {
                write!(f, "{:>4}", format!("({})", i))?;
            } else {
                write!(f, " {:>2} ", i)?;
            }
            i = ll.1;
            if i == top {
                break;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let mut c = Circle::new(9);
        assert_eq!(c.current, 0);
        assert_eq!(c.top(), 0);
        assert_eq!(format!("{:?}", c), " (0)");

        c.step();
        assert_eq!(format!("{:?}", c), "  0  (1)");
        c.step();
        assert_eq!(format!("{:?}", c), "  0  (2)  1 ");
        c.step();
        assert_eq!(format!("{:?}", c), "  0   2   1  (3)");
        c.step();
        assert_eq!(format!("{:?}", c), "  0  (4)  2   1   3 ");

        while c.current < 22 {
            c.step();
        }
        assert_eq!(format!("{:?}", c),
            "  0  16   8  17   4  18   9  19   2  20  10  21   5 (22) 11   1  12   6  13   3  14   7  15 ");
        c.step();
        assert_eq!(format!("{:?}", c),
            "  0  16   8  17   4  18 (19)  2  20  10  21   5  22  11   1  12   6  13   3  14   7  15 ");
        assert_eq!(c.scores, vec![0, 0, 0, 0, 32, 0, 0, 0, 0]);
        assert_eq!(c.high_score(), 32);
    }

    #[test]
    fn try_1618() {
        // 10 players; last marble is worth 1618 points: high score is 8317
        assert_eq!(Circle::new(10).play_until(1618).high_score(), 8317);
    }

    #[test]
    fn try_7999() {
        // 13 players; last marble is worth 7999 points: high score is 146373
        assert_eq!(Circle::new(13).play_until(7999).high_score(), 146373);
    }

    #[test]
    fn try_6111() {
        // 21 players; last marble is worth 6111 points: high score is 54718
        assert_eq!(Circle::new(21).play_until(6111).high_score(), 54718);
    }

    #[test]
    fn try_5087() {
        // 30 players; last marble is worth 5807 points: high score is 37305
        assert_eq!(Circle::new(30).play_until(5807).high_score(), 37305);
    }

    #[test]
    fn try_1104() {
        // 17 players; last marble is worth 1104 points: high score is 2764
        assert_eq!(Circle::new(17).play_until(1104).high_score(), 2764);
    }
}
