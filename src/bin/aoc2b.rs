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
use std::io;
use std::io::prelude::*;

/// True if the two strings differ by exactly one character at the same position
fn onediff(a: &str, b: &str) -> bool {
    a.chars().zip(b.chars()).filter(|(aa, bb)| aa != bb).count() == 1
}

/// Common characters between two strings
fn common(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter_map(|(aa, bb)| if aa == bb { Some(aa) } else { None })
        .collect()
}

fn find_close<S: AsRef<str>>(ls: Vec<S>) -> Option<String> {
    for i in ls.iter() {
        for j in ls.iter() {
            let i = i.as_ref();
            let j = j.as_ref();
            if onediff(i, j) {
                return Some(common(i, j));
            }
        }
    }
    None
}

pub fn main() {
    let ls: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    println!("{}", find_close(ls).unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_onediff() {
        use super::onediff;

        assert_eq!(onediff("abcde", "axcye"), false);
        assert_eq!(onediff("abcde", "wvxyz"), false);
        assert_eq!(onediff("fghij", "axcye"), false);
        assert_eq!(onediff("fghij", "fguij"), true);
    }

    #[test]
    fn find_close() {
        use super::find_close;
        assert_eq!(
            find_close(vec![
                "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
            ]),
            Some("fgij".to_string())
        );
    }
}
