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

//! https://adventofcode.com/2018/day/25
//!
//! Find constellations based on 4d Manhattan distance between points.

type Point = [isize; 4];
const NEAR: isize = 3;

fn distance(a: &Point, b: &Point) -> isize {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs() + (a[3] - b[3]).abs()
}

fn is_near(a: &Point, b: &Point) -> bool {
    distance(a, b) <= NEAR
}

fn parse_string(s: &str) -> Vec<Point> {
    let mut r = Vec::new();
    for l in s.lines() {
        let mut li = l.trim().split(',').map(|s| s.parse().unwrap());
        r.push([
            li.next().unwrap(),
            li.next().unwrap(),
            li.next().unwrap(),
            li.next().unwrap(),
        ]);
    }
    r
}

fn load_input() -> Vec<Point> {
    parse_string(&std::fs::read_to_string("input/input25.txt").unwrap())
}

fn near_constellation(p: &Point, c: &[Point]) -> bool {
    c.iter().any(|q| is_near(p, q))
}

/// Cluster a group of points into constellations that are each no more
/// than NEAR Manhattan distance from at least one other group in the
/// constellation.
fn constellations(pts: &[Point]) -> Vec<Vec<Point>> {
    let mut cts: Vec<Vec<Point>> = Vec::new();
    for p in pts {
        // There are three basic possibilities:
        //
        // 1. It's not near any already-known constellations: this is trivially
        // true for the first point we look at since there are none. We make a new
        // constellation.
        //
        // 2. It's near at least one point from only one existing constellation.
        // We add it to that constellation and we're done.
        //
        // 3. It's near more than one existing constellation: we add it to the
        // first and then fuse together all the others that we find.

        // First, find the indexes of existing constellations which are near this point.
        let ii: Vec<usize> = cts
            .iter()
            .enumerate()
            .filter(|(_i, c)| near_constellation(p, &c))
            .map(|(i, _c)| i)
            .collect();

        if ii.is_empty() {
            cts.push(vec![*p]);
        } else {
            cts[ii[0]].push(*p);
            for j in ii[1..].iter().rev() {
                // Consume them in reverse order so that the indexes don't change until
                // we get to them.
                let oc = cts.remove(*j);
                cts[ii[0]].extend_from_slice(&oc);
            }
        }
    }
    cts
}

fn solve_a() -> usize {
    let pts = load_input();
    constellations(&pts).len()
}

pub fn main() {
    println!("A: {}", solve_a());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let pts = parse_string(
            "\
            0,0,0,0
            3,0,0,0
            0,3,0,0
            0,0,3,0
            0,0,0,3
            0,0,0,6
            9,0,0,0
            12,0,0,0",
        );
        assert_eq!(pts.len(), 8);
        let cts = constellations(&pts);
        assert_eq!(cts.len(), 2);
        assert_eq!(cts[0].len(), 6);
        assert_eq!(cts[1].len(), 2);
    }

    #[test]
    fn example_1b() {
        let pts = parse_string(
            "\
            0,0,0,0
            3,0,0,0
            0,3,0,0
            0,0,3,0
            0,0,0,3
            0,0,0,6
            9,0,0,0
            12,0,0,0
            6,0,0,0",
        );
        assert_eq!(pts.len(), 9);
        let cts = constellations(&pts);
        assert_eq!(cts.len(), 1);
        assert_eq!(cts[0].len(), 9);
    }

    #[test]
    fn example_2() {
        let pts = parse_string(
            "-1,2,2,0
            0,0,2,-2
            0,0,0,-2
            -1,2,0,0
            -2,-2,-2,2
            3,0,2,-1
            -1,3,2,2
            -1,0,-1,0
            0,2,1,-2
            3,0,0,0",
        );
        let cts = constellations(&pts);
        assert_eq!(cts.len(), 4);
    }

    #[test]
    fn example_3() {
        let pts = parse_string(
            "\
            1,-1,0,1
            2,0,-1,0
            3,2,-1,0
            0,0,3,1
            0,0,-1,-1
            2,3,-2,0
            -2,2,0,0
            2,-2,0,-1
            1,-1,0,-1
            3,2,0,2",
        );
        let cts = constellations(&pts);
        assert_eq!(cts.len(), 3);
    }

    #[test]
    fn example_4() {
        let pts = parse_string(
            "\
            1,-1,-1,-2
            -2,-2,0,1
            0,2,1,3
            -2,3,-2,1
            0,2,3,-2
            -1,-1,1,-2
            0,-2,-1,0
            -2,2,3,-1
            1,2,2,0
            -1,-2,0,-2",
        );
        let cts = constellations(&pts);
        assert_eq!(cts.len(), 8);
    }

    #[test]
    fn load_input() {
        super::load_input();
    }

    #[test]
    fn known_solution_a() {
        assert_eq!(solve_a(), 390);
    }
}
