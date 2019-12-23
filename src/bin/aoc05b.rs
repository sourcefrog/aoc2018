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

//! https://adventofcode.com/2018/day/5
use std::io;
use std::io::prelude::*;
use std::iter::Iterator;

pub fn main() {
    let lines = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    assert_eq!(lines.len(), 1);
    let l = &lines[0];
    let mut best_len = usize::max_value();
    let mut best_char = 0u8;
    for t in b'a'..=b'z' {
        let c = collapse_without(&l, t as char);
        println!("Remove {:?} => len {}", t as char, c.len());
        if c.len() < best_len {
            best_len = c.len();
            best_char = t;
        }
    }
    println!(
        "Best to remove: {:?} => len {}",
        best_char as char, best_len
    );
}

fn matches(c0: char, c1: char) -> bool {
    c0.to_ascii_lowercase() == c1.to_ascii_lowercase()
        && c0.is_ascii_uppercase() != c1.is_ascii_uppercase()
}

/// Repeatedly remove matched letters from s; return the remnant. Ignore
/// upper or lower case t.
fn collapse_without(s: &str, t: char) -> String {
    // Keep a stack of as-yet unmatched characters. Consume from the input
    // one character at a time; if it matches the top character from the stack
    // then pop that off (and also discard the new one); otherwise push.
    assert!(t.is_ascii_lowercase());
    let mut st: Vec<char> = Vec::new();
    for c1 in s.chars() {
        if c1.to_ascii_lowercase() == t {
            continue;
        } else if let Some(c0) = st.pop() {
            if matches(c0, c1) {
                // println!("{:?} ** {:?}", c0, c1);
                // discard both
            } else {
                st.push(c0);
                st.push(c1);
            }
        } else {
            // Stack's already empty; must push
            st.push(c1);
        }
    }
    st.into_iter().collect()
}
