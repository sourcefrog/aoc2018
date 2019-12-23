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

// Recipes are only ever appended to the board.

pub fn main() {
    println!("{}", Board::new().recipes_before(&[8, 9, 0, 6, 9, 1]));
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Board {
    r: Vec<u8>,

    /// Index of the current recipe of Elf 1.
    e1: usize,
    e2: usize,
}

impl Board {
    pub fn new() -> Board {
        let mut r = vec![3, 7];
        r.reserve(30_000_000);
        Board { r, e1: 0, e2: 1 }
    }

    pub fn step(&mut self) {
        // Sum the current recipes
        let s = self.r[self.e1] + self.r[self.e2];
        // Create new recipes
        if s >= 10 {
            self.r.push(1);
            self.r.push(s % 10);
        } else {
            self.r.push(s);
        }
        // Advance
        self.e1 = (self.e1 + 1 + self.r[self.e1] as usize) % self.r.len();
        self.e2 = (self.e2 + 1 + self.r[self.e2] as usize) % self.r.len();
    }

    /// Return the number of recipes (r items) before the suffix of the list is s.
    /// In doing this take care that we can add one or two recipes at a time.
    pub fn recipes_before(&mut self, s: &[u8]) -> usize {
        let sl = s.len();
        loop {
            let rl = self.r.len();
            // TODO: This does duplicate checks when `r` grows by only one element,
            // but that should be fairly rare.
            if rl > (sl + 1) && self.r[(rl - sl - 1)..(rl - 1)] == *s {
                return rl - sl - 1;
            } else if rl > sl && self.r[(rl - sl)..rl] == *s {
                return rl - sl;
            }
            self.step();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(Board::new().recipes_before(&[5, 1, 5, 8, 9]), 9);
        assert_eq!(Board::new().recipes_before(&[0, 1, 2, 4, 5]), 5);
        assert_eq!(Board::new().recipes_before(&[9, 2, 5, 1, 0]), 18);
        assert_eq!(Board::new().recipes_before(&[5, 9, 4, 1, 4]), 2018);
    }
}
