#![allow(dead_code)]

//! https://adventofcode.com/2018/day/23
//!
//! Cloud of nanobots able to teleport things within a distance of their
//! (x,y,z) position.
//!
//! Really, this is about finding intersections between Manhattan-distance
//! diamond shapes in 3d space.

// Part two requires finding the position that's in range of the largest number
// of nanobots.
//
// There are a thousand nanobots in the input, distributed over a fairly wide
// range of inputs (coordinates on the order of ~1M points) and also with large
// ranges. So it seems any kind of brute force search on individual points is
// infeasible.
//
// One approach would be to track the set of constraints on the intersection
// between the zone of two bots, and then gradually see if we can also reach
// any other bots.
//
// So the question then is, is there a tractable representation of the shape of
// the intersection between two bots? And more than two bots?
//
// In 2D the Manhattan-distance space will be a diamond-shape with edges on
// slope x+y = +/- 1. Similarly in 3d, with planes of unit slope.
//
// Since we're always looking for the intersection of these constraints, I
// suspect the constraints will always keep it a simple convex diamond.
//
// Then after having a way to define these shapes, we have to look for the
// largest subset of bots having a non-empty intersection. The naive approach
// would be to test all 2**1000 possibilities, which is also infeasible. But,
// actually, we can often terminate early if we find there is no intersection
// for some subset. And we can abandon some possibilites where they cannot
// possibly become the longest.
//
// A Bot {x,y,z,r} can reach points (X,Y,Z) where
//
// (X-x).abs() + (Y-y).abs() + (Z-z).abs() <= r
//
// Start with Z=z to reduce it to the 2D case. Also, start at the X>x, Y>y
// case, so the abs terms go away.
//
// (X - x) + (Y - y) <= r X + Y <= r + x + y
//
// then if X<x, Y>y (x-X) + (Y-y) <= r -X + Y <= r - x + y
//
// similarly X - Y <= r + x - y -X - Y <= r - x - y
//
// These four constraints define a quadrilateral with unit slopes in a plane of
// Z=z.
//
// Expading to Z>z,
//
//  X + Y + Z   <= r + x + y + z -X + Y + Z   <= r - x + y + z X - Y + Z   <= r
//  + x - y + z ...
//
// So there are eight planes constraining the space, and they're all defined by
// simple combinations of (x,y,z,r). Of course, they have to be.
//
// How do could we intersect two zones to find whether there is any resulting
// zone?  First consider the simple one-dimensional case, (x1,r1) and (x2,r2)
// where x1 <= x2.
//
// If (x1+r1) < (x2-r2) they do not touch; there is no intersection.
//
// aaaaAaaaa             x1 = 4, r1 = 4 bbbbbbBbbbbbb   x2 = 12, r2 = 6
//
// Otherwise, there is an intersection of length di=((x1+r1) - (x2-r2)).  and
// with radius (treating the edge as included) of ri = di/2.  Implies r1 = (x1
// + r1 - x2 + r2) / 2.  The center is at (x1 + r1 - ri).
//
// This needs care with regard to off-by-one errors. How do we cope if di is
// even? Maybe in that case it cannot be represented as (x,y,z,r)?
//
// And, in fact, this has another unhandled edge case: suppose x1=x2 but r2>r1.
//
// Perhaps it is actually easiest to represent zones as the inclusive ranges of
// coordinates, and then at least the math to calculate the intersections is
// simple.
//
// So in the case given above, for the A range, x>=0, x<=8. For B, x>=6, x<=18.
// The intersection is simply x>=6, x<=8. In other words ximin=max(xamin,
// xbmin). ximax=min(xamax, xbmax).
//
// Instead of using both >= and <= we could say: A(x <= 8, -x <= 0). B(x <= 18,
// -x <= -6).
//
// How does this extend to 2, and to 3, dimensions?
//
// Then for 2 dimensions give maximum values of (x+y), (-x+y), (x-y), (-x-y).
//
// Similarly for 3 dimesions, the maximum values of all eight combinations.
//
// Perhaps there's a simpler expression than writing them all out. Not sure.
//
// Now, how can we tell if the zone is empty? Equivalently, r<0?
//
// pxpypz = x + y + z + r mxmymz = -x - y - z + r
//
// pxpypz + mxmymz = 2r
//
// Now, moving on to finding the zone, and the closest-to-origin point, that
// intersects the zones of the largest number of bots.
//
// One way to approach this is to find the zones that intersect between all
// combinations of bots, pruning off combinations that have no intersection.
// The problem with this is that there are up to 2**1000 possible combinations
// of bots, and there are many overlaps between them, so not much can be pruned
// out.
//
// We could take the same approach but work from the other end and see how many
// need to be removed to find an intersection, although that might also blow
// up.
//
// I wonder if it would help to cluster bots at similar locations, or deal
// first with the largest ones?
//
// So we can consider this as a function over x,y,z where h(x,y,z) gives the
// number of bots in range at any point. We want to find the point with the
// maximum x,y,z. The values can be considered to stack up as we go along.
//
// There are vary many possible x,y,z because the ranges are so large, but only
// some of them can be interesting: the ones at the edge of a zone boundary. No
// new ones can possibly be introduced.
//
// (In fact, is it only the edges closest to the origin that are interesting?)
//
// Each bot defines a diamond with planes defined by eight parameters. The
// diamond has six corners.  Are the interesting points always at one of these
// corners? If so, perhaps it's easy to see which other bots might contain
// them.
//
// 82010378 is too low...
//
// Just checking the corners, it seems, is not enough: it gets an answer that
// is too low (too close to the origin.) Perhaps, there's another position
// further away that intersects with more diamonds.
//
// Perhaps checking only the corners is not enough: when there are multiple
// overlapping squares I can roughly imagine how the most-covered point
// wouldn't be exactly on a corner. Must it be on an edge? It would be nice to
// have a rigorous argument that it must.
//
// Return to the concept of drawing a graph along one axis, and looking for the
// maximum stacking. It still seems it must occur on corners?
//
// I wonder if I should be looking for intersections between all the planes,
// rather than just the corners? But, I can't yet think of any case where that
// demonstrates any difference, or where the positions could be other than at
// all the corners.

extern crate itertools;
extern crate regex;

use std::cmp::min;
use std::fs::File;
use std::io::Read;

// use itertools::Itertools;

use regex::Regex;

/// The location and radius of one nanobot.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Bot {
    x: isize,
    y: isize,
    z: isize,
    r: isize,
}

impl Bot {
    fn zone(&self) -> Zone {
        Zone {
            pxpypz: self.x + self.y + self.z + self.r,
            pxpymz: self.x + self.y - self.z + self.r,
            pxmypz: self.x - self.y + self.z + self.r,
            pxmymz: self.x - self.y - self.z + self.r,
            mxpypz: -self.x + self.y + self.z + self.r,
            mxpymz: -self.x + self.y - self.z + self.r,
            mxmypz: -self.x - self.y + self.z + self.r,
            mxmymz: -self.x - self.y - self.z + self.r,
        }
    }

    fn corners(&self) -> [(isize, isize, isize); 6] {
        [
            (self.x - self.r, self.y, self.z),
            (self.x + self.r, self.y, self.z),
            (self.x, self.y - self.r, self.z),
            (self.x, self.y + self.r, self.z),
            (self.x, self.y, self.z - self.r),
            (self.x, self.y, self.z + self.r),
        ]
    }

    fn contains_point(&self, p: (isize, isize, isize)) -> bool {
        (self.x - p.0).abs() + (self.y - p.1).abs() + (self.z - p.2).abs() <= self.r
    }
}

/// Parse an input string containing bot position descriptions into a vec of
/// Bots.
fn parse(s: &str) -> Vec<Bot> {
    let re: Regex = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    s.lines()
        .map(|l| {
            if let Some(caps) = re.captures(l) {
                let fld = |i| caps.get(i).unwrap().as_str().parse().unwrap();
                Bot {
                    x: fld(1),
                    y: fld(2),
                    z: fld(3),
                    r: fld(4),
                }
            } else {
                panic!("failed to parse: {:?}", l);
            }
        })
        .collect()
}

/// Return the strongest Bot, which is the one with the largest radius.
fn strongest(bs: &[Bot]) -> Bot {
    *bs.iter().max_by_key(|b| b.r).unwrap()
}

/// Return the Manhattan distance between two bots.
fn distance(a: &Bot, b: &Bot) -> isize {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
}

/// Return the number of bots in range of the strongest bot (including itself.)
fn count_in_range(bs: &[Bot]) -> usize {
    let stz = strongest(&bs).zone();
    bs.iter().filter(|b| stz.contains(&b)).count()
}

/// Load bots from input file.
fn load_input() -> Vec<Bot> {
    let mut s = String::with_capacity(50_000);
    File::open("input/input23.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    parse(&s)
}

/// The teleportation zone of one or more bots.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Zone {
    // The zone is expressed as the inclusive maximum value of a sum of the
    // positive or negative values of x, y, and z. Within the zone all of these
    // constraints are satisfied.
    pxpypz: isize,
    pxpymz: isize,
    pxmypz: isize,
    pxmymz: isize,
    mxpypz: isize,
    mxpymz: isize,
    mxmypz: isize,
    mxmymz: isize,
}

impl Zone {
    fn contains(&self, b: &Bot) -> bool {
        self.contains_point((b.x, b.y, b.z))
    }

    fn contains_point(&self, b: (isize, isize, isize)) -> bool {
        (b.0 + b.1 + b.2) <= self.pxpypz
            && (b.0 + b.1 - b.2) <= self.pxpymz
            && (b.0 - b.1 + b.2) <= self.pxmypz
            && (b.0 - b.1 - b.2) <= self.pxmymz
            && (-b.0 + b.1 + b.2) <= self.mxpypz
            && (-b.0 + b.1 - b.2) <= self.mxpymz
            && (-b.0 - b.1 + b.2) <= self.mxmypz
            && (-b.0 - b.1 - b.2) <= self.mxmymz
    }

    fn intersect(&self, other: &Zone) -> Zone {
        Zone {
            pxpypz: min(self.pxpypz, other.pxpypz),
            pxpymz: min(self.pxpymz, other.pxpymz),
            pxmypz: min(self.pxmypz, other.pxmypz),
            pxmymz: min(self.pxmymz, other.pxmymz),
            mxpypz: min(self.mxpypz, other.mxpypz),
            mxpymz: min(self.mxpymz, other.mxpymz),
            mxmypz: min(self.mxmypz, other.mxmypz),
            mxmymz: min(self.mxmymz, other.mxmymz),
        }
    }

    /// True if the constraints on this zone imply it covers no space.
    fn is_empty(&self) -> bool {
        (self.pxpypz + self.mxmymz) < 0
            || (self.pxpymz + self.mxmypz) < 0
            || (self.pxmypz + self.mxpymz) < 0
            || (self.pxmymz + self.mxpypz) < 0
    }

    /// Returns four "radiuses" along each axis. If 0, there's only one
    /// possible value on that axis; if negative there are no possible values.
    fn r_values(&self) -> (isize, isize, isize, isize) {
        (
            self.pxpypz + self.mxmymz,
            self.pxpymz + self.mxmypz,
            self.pxmypz + self.mxpymz,
            self.pxmymz + self.mxpypz,
        )
    }
}

/// Solve part A from real input.
fn solve_a() -> usize {
    count_in_range(&load_input())
}

fn solve_b() -> isize {
    let bots = load_input();
    let bp = find_most_covered(&bots);
    bp.0.abs() + bp.1.abs() + bp.2.abs()
}

fn find_most_covered(bots: &[Bot]) -> (isize, isize, isize) {
    let mut best_points = Vec::new();
    let mut best_matches = 0;

    // Number of matches at that point, and the point.
    let mut res: Vec<(usize, (isize, isize, isize))> = Vec::new();

    for (_i, b) in bots.iter().enumerate() {
        // dbg!(i, b);
        for corn in b.corners().iter() {
            debug_assert!(b.contains_point(*corn));
            let matches = bots.iter().filter(|b| b.contains_point(*corn)).count();
            res.push((matches, *corn));
            // dbg!(corn, matches);
            if matches > best_matches {
                best_points.clear();
                best_points.push(*corn);
                best_matches = matches;
            } else if matches == best_matches {
                best_points.push(*corn);
            }
        }
    }
    res.sort();
    res.reverse();
    dbg!(&res[..10]);
    dbg!(&best_points, &best_matches);
    assert_eq!(best_points.len(), 1);
    let bp = best_points[0];
    dbg!(bp.0 + bp.1 + bp.2);
    bp
}

pub fn main() {
    // dbg!(solve_a());
    println!("Solution to B: {}", solve_b());
}

#[cfg(test)]
mod tests {
    use super::Bot;

    use itertools::Itertools;

    #[test]
    fn example_1() {
        let t = "\
            pos=<0,0,0>, r=4
            pos=<1,0,0>, r=1
            pos=<4,0,0>, r=3
            pos=<0,2,0>, r=1
            pos=<0,5,0>, r=3
            pos=<0,0,3>, r=1
            pos=<1,1,1>, r=1
            pos=<1,1,2>, r=1
            pos=<1,3,1>, r=1\
            ";
        let bots = super::parse(t);
        assert_eq!(bots.len(), 9);
        assert_eq!(
            bots[0],
            Bot {
                x: 0,
                y: 0,
                z: 0,
                r: 4
            }
        );
        assert_eq!(
            bots[8],
            Bot {
                x: 1,
                y: 3,
                z: 1,
                r: 1,
            }
        );
        assert_eq!(
            super::strongest(&bots),
            Bot {
                x: 0,
                y: 0,
                z: 0,
                r: 4,
            }
        );
        assert_eq!(super::count_in_range(&bots), 7);
    }

    #[test]
    fn expected_result() {
        assert_eq!(super::solve_a(), 232);
    }

    #[test]
    fn test_intersect() {
        let v = "\
            pos=<10,12,12>, r=2
            pos=<12,14,12>, r=2
            pos=<16,12,12>, r=4
            pos=<14,14,14>, r=6
            pos=<50,50,50>, r=200\
            ";
        // Only the coordinate (12,12,12) is in range of all five of these.
        let bots = super::parse(v);
        let inter_zone = bots
            .iter()
            .map(|b| b.zone())
            .fold1(|az, bz| az.intersect(&bz))
            .unwrap();
        dbg!(inter_zone);
        dbg!(inter_zone.r_values());
        assert!(inter_zone.contains_point((12, 12, 12)));
        assert_eq!(inter_zone.r_values(), (0, 0, 0, 0));
    }

    #[test]
    fn example_2() {
        let v = "\
        pos=<10,12,12>, r=2
        pos=<12,14,12>, r=2
        pos=<16,12,12>, r=4
        pos=<14,14,14>, r=6
        pos=<50,50,50>, r=200
        pos=<10,10,10>, r=5\
        ";
        let bots = super::parse(v);
        assert_eq!(super::find_most_covered(&bots), (12, 12, 12));
    }

}
