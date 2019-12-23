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

//! https://adventofcode.com/2018/day/8
use std::io;
use std::io::prelude::*;

pub fn main() {
    let nums = parse_ints(&io::stdin().lock().lines().next().unwrap().unwrap());
    println!("{:?}", nums);
    println!(
        "sum of metadata = {:?}",
        sum_metadata(&mut nums.into_iter())
    )
}

/// Read a node and any child nodes out of the iterator, and return the sum of
/// their metadata.
fn sum_metadata<I: Iterator<Item = u32>>(l: &mut I) -> u32 {
    let nkids = l.next().unwrap();
    let nmeta = l.next().unwrap();
    let mut tot = 0;
    for _i in 0..nkids {
        tot += sum_metadata(l);
    }
    for _i in 0..nmeta {
        tot += l.next().unwrap();
    }
    tot
}

pub fn parse_ints(s: &str) -> Vec<u32> {
    s.split(' ')
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let nums = parse_ints("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(sum_metadata(&mut nums.into_iter()), 138);
    }
}
