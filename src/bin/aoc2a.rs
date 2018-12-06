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

/// https://adventofcode.com/2018/day/2
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

/// Say whether s has, respectively
/// - any letters that occur exactly twice each
/// - any letters that occur exactly three times
fn pat(s: &str) -> (bool, bool) {
    let mut counts = HashMap::<char, u32>::new();
    for c in s.chars() {
        counts.insert(c, counts.get(&c).unwrap_or(&0) + 1);
    }
    // Were there any characters present exactly 2 times, or exactly 3?
    (
        counts.values().any(|x| *x == 2),
        counts.values().any(|x| *x == 3),
    )
}

fn checksum<S: AsRef<str>>(ss: &[S]) -> i64 {
    let pats: Vec<(bool, bool)> = ss.iter().map(|sp| pat(sp.as_ref())).collect();
    pats.iter().filter(|(two, _)| *two).count() as i64
        * pats.iter().filter(|(_, three)| *three).count() as i64
}

pub fn main() {
    let ls: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    println!("checksum: {}", checksum(ls.as_slice()));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pat() {
        assert_eq!(super::pat("hello"), (true, false));
        assert_eq!(super::pat("abcdef"), (false, false));
        assert_eq!(super::pat("bababc"), (true, true));
        assert_eq!(super::pat("abbcde"), (true, false));
        assert_eq!(super::pat("abcccd"), (false, true));
        assert_eq!(super::pat("aabcdd"), (true, false));
        assert_eq!(super::pat("abcdee"), (true, false));
        assert_eq!(super::pat("ababab"), (false, true));
    }

    #[test]
    fn test_checksum() {
        assert_eq!(
            super::checksum(&[
                "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"
            ]),
            12
        );
    }
}
