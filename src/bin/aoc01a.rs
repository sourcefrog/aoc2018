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

//! https://adventofcode.com/2018/day/1

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
