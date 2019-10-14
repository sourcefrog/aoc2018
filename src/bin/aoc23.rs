//! https://adventofcode.com/2018/day/23
//!
//! Cloud of nanobots able to teleport things within a distance of their
//! (x,y,z) position.
//!
//! Really, this is about finding intersections between Manhattan-distance
//! diamond shapes in 3d space.
//!
//! The basic problem is NP-hard, and n=1000, so the challenge here is to get
//! some reasonable approximation that we can actually compute.

extern crate itertools;
extern crate regex;

use std::cmp::{max, min};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;

use regex::Regex;

type Coord = (isize, isize, isize);

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

    #[allow(unused)]
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

    #[allow(unused)]
    fn contains_point(&self, p: (isize, isize, isize)) -> bool {
        (self.x - p.0).abs() + (self.y - p.1).abs() + (self.z - p.2).abs() <= self.r
    }

    /// Return the Manhattan distance between two bots.
    #[allow(unused)]
    fn distance(&self, b: &Bot) -> isize {
        (self.x - b.x).abs() + (self.y - b.y).abs() + (self.z - b.z).abs()
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

    /// Return a coordinate in this zone that's closest to the origin.
    fn closest_to_origin(&self) -> Coord {
        let xmax1 = (self.pxpypz + self.pxmymz) / 2;
        println!("x <= {}", xmax1);
        let xmax2 = (self.pxmypz + self.pxpymz) / 2;
        println!("x <= {}", xmax2);
        let xmin1 = -(self.mxpypz + self.mxmymz) / 2;
        println!("x >= {}", xmin1);
        let xmin2 = -(self.mxmypz + self.mxpymz) / 2;
        println!("x >= {}", xmin2);

        if xmin1 < 0 || xmin2 < 0 {
            // If negative we ought to look at the maximums.
            unimplemented!();
        }
        let x = max(xmin1, xmin2);
        println!("therefore x={}", x);

        // x + y + z <= pxpypz
        // x + y - z <= pxpymz
        // 2y <= pxpypz + pxpymz - 2x
        let ymax1 = (self.pxpypz + self.pxpymz) / 2 - x;
        let ymax2 = (self.mxpypz + self.mxpymz) / 2 + x;
        dbg!(ymax1, ymax2);

        // x - y + z <= pxmypz
        // x - y - z <= pxmymz
        // 2x -2y <= pxmypz + pxmymz
        // -2y <= pxmypz + pxmymz - 2x
        // y >= -(pxmypz + pxmymz) / 2 + x
        let ymin1 = -(self.pxmypz + self.pxmymz) / 2 + x;
        let ymin2 = -(self.mxmypz + self.mxmymz) / 2 - x;
        dbg!(ymin1, ymin2);
        if ymin1 < 0 || ymin2 < 0 {
            unimplemented!();
        }
        let y = max(ymin1, ymin2);
        dbg!(y);

        // -x -y -z <= mxmymz
        // -z <= mxmymz + x + y
        // z >= -mxmymz - x - y
        let zmin1 = -self.mxmymz - x - y;
        // x + y - z <= pxpymz
        // -z <= pxpymz - x - y
        // z >= -pxpymz + x + y
        let zmin2 = -self.pxpymz + x + y;
        dbg!(zmin1, zmin2);
        let z = max(zmin1, zmin2);

        let p = (x, y, z);
        dbg!(p);
        dbg!(p.0.abs() + p.1.abs() + p.2.abs());

        assert!(self.contains_point(p));

        p
    }
}

/// Solve part A from real input.
fn solve_a() -> usize {
    count_in_range(&load_input())
}

fn distance_from_origin(p: Coord) -> isize {
    p.0.abs() + p.1.abs() + p.2.abs()
}

// Return the number of bots that can reach p.
#[allow(unused)]
fn count_coverage(bots: &[Bot], p: Coord) -> usize {
    bots.iter().filter(|b| b.contains_point(p)).count()
}

// Return the distance from the origin of the point closest to the origin that is covered by the
// most bots, and that point.
fn find_most_covered(bots: &[Bot]) -> (isize, Coord) {
    // `cov[i]` is a set of all so-far-known zones covered by `i+1`
    // bots.
    let mut cov: Vec<BTreeSet<Zone>> = vec![BTreeSet::new(); bots.len()];

    for (i, b) in bots.iter().enumerate() {
        let mut limit = 1000;
        // dbg!(i, b);
        // For every previously-found cover, let's add b to it, to see if we can generate some
        // higher-level covering zones.
        let z = b.zone();
        'bot: for j in (0..i).rev() {
            let newent: Vec<Zone> = cov[j]
                .iter()
                .map(|oz| oz.intersect(&z))
                .filter(|iz| !iz.is_empty())
                .collect();
            // dbg!(j + 1, &newent.len());
            for nz in newent {
                // If this exact same zone is already in level j, remove it: we only need to
                // track it at j+1.
                cov[j].remove(&nz);
                cov[j + 1].insert(nz);
                limit -= 1;
                if limit == 0 {
                    break 'bot;
                }
            }
            if j + 1 == i {
                // dbg!(&cov[j + 1]);
            }
        }

        // Now let's add new layer 0 zones, covered only by this.
        cov[0].insert(z);
    }

    // TODO: The highest populated value in `cov` is the solution.
    for j in (0..cov.len()).rev() {
        if !cov[j].is_empty() {
            dbg!(j, &cov[j]);
            let best_point = cov[j].iter().next().unwrap().closest_to_origin();
            return (distance_from_origin(best_point), best_point);
        }
    }
    unreachable!();
}

fn solve_b() -> isize {
    let bots = load_input();

    // Make a list of, for each bot, the identities of other bots that touch it.
    let mut touchs: Vec<BTreeSet<usize>> = vec![Default::default(); bots.len()];

    for (i, a) in bots.iter().enumerate() {
        for (j, b) in bots.iter().enumerate() {
            if !a.zone().intersect(&b.zone()).is_empty() {
                touchs[i].insert(j);
            }
        }
    }

    // Find the largest `m` such that there are at least `m` bots that each intersect
    // at least `m` bots.
    let mut tc: Vec<_> = touchs.iter().map(|t| t.len()).collect();
    tc.sort();
    let m = tc
        .iter()
        .filter(|i| tc.iter().filter(|t| t >= i).count() >= **i)
        .max()
        .expect("Found no maximum likely clique");
    dbg!(m);

    // Find the specific bots that touch at least `m` bots.
    let included_bots: Vec<Bot> = touchs
        .iter()
        .enumerate()
        .filter(|(_b, t)| t.len() >= *m)
        .map(|(b, _t)| bots[b])
        .collect();

    // (I'm not sure this necessarily must be true, but it is true on this input.)
    assert_eq!(included_bots.len(), *m);

    let excluded_bots: Vec<Bot> = bots
        .iter()
        .filter(|b| !included_bots.contains(b))
        .copied()
        .collect();
    assert_eq!(excluded_bots.len(), bots.len() - m);

    // Now (hopefully) find a region that's common between all those bots.
    let intersection_zone = included_bots
        .iter()
        .fold(None, |ac, bot| match ac {
            None => Some(bot.zone()),
            Some(z) => Some(bot.zone().intersect(&z)),
        })
        .unwrap();
    dbg!(intersection_zone);

    // Let's check none of the excluded bots overlap with this region. It doesn't prove
    // it's the largest possible region, but it does prove it overlaps with exactly
    // `m` bots.
    assert!(excluded_bots
        .iter()
        .all(|b| b.zone().intersect(&intersection_zone).is_empty()));

    distance_from_origin(intersection_zone.closest_to_origin())
}

pub fn main() {
    println!("Solution to A: {}", solve_a());
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
    fn expected_result_a() {
        assert_eq!(super::solve_a(), 232);
    }

    #[test]
    fn expected_result_b() {
        assert_eq!(super::solve_b(), 82010396);
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
        assert_eq!(super::find_most_covered(&bots), (36, (12, 12, 12)));
    }
}
