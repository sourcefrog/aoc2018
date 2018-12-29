#![allow(dead_code)]

use std::collections::BTreeMap;

/// https://adventofcode.com/2018/day/15
use aoc2018::matrix::Matrix;
use aoc2018::Point;

const INITIAL_HP: usize = 200;
const ATTACK_POWER: usize = 3;

pub fn main() {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Thing {
    Empty,
    Wall,
    Elf,
    Goblin,
}

impl Thing {
    pub fn from_char(ch: char) -> Thing {
        match ch {
            '.' => Thing::Empty,
            '#' => Thing::Wall,
            'E' => Thing::Elf,
            'G' => Thing::Goblin,
            other => panic!("unexpected character {:?}", other),
        }
    }

    /// The enemy race for creatures (only).
    pub fn enemy(&self) -> Thing {
        match self {
            Thing::Goblin => Thing::Elf,
            Thing::Elf => Thing::Goblin,
            _ => panic!("not a creature: {:?}", self),
        }
    }

    pub fn is_empty(self) -> bool {
        self == Thing::Empty
    }

    pub fn is_creature(self) -> bool {
        self == Thing::Elf || self == Thing::Goblin
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Creature {
    p: Point,
    race: Thing,
    hp: usize,
}

struct Map {
    m: Matrix<Thing>,
    w: usize,
    h: usize,
    cs: BTreeMap<Point, Creature>,
}

impl Map {
    pub fn from_string(s: &str) -> Map {
        let mut mb = Matrix::<Thing>::from_rows();
        let mut cs = BTreeMap::<Point, Creature>::new();
        for (y, l) in s.lines().enumerate() {
            let r: Vec<Thing> = l.chars().map(Thing::from_char).collect();
            for (x, &th) in r.iter().enumerate() {
                let p = Point { y, x };
                if th.is_creature() {
                    cs.insert(
                        p,
                        Creature {
                            p,
                            race: th,
                            hp: INITIAL_HP,
                        },
                    );
                }
            }
            mb.add_row(&r);
        }
        let m = mb.finish();
        Map {
            cs,
            w: m.width(),
            h: m.height(),
            m,
        }
    }

    /// Return all valid neighbors of a point, in reading order.
    pub fn neighbors(&self, p: Point) -> Vec<Point> {
        let mut v = Vec::with_capacity(4);
        let x = p.x;
        let y = p.y;
        if y > 0 {
            v.push(Point { x, y: y - 1 })
        }
        if x > 0 {
            v.push(Point { x: x - 1, y })
        }
        if x < self.w - 1 {
            v.push(Point { x: x + 1, y })
        }
        if y < self.h - 1 {
            v.push(Point { x, y: y + 1 })
        }
        v
    }

    /// Return the empty neighbors of a point, in reading order.
    pub fn empty_neighbors(&self, p: Point) -> Vec<Point> {
        self.neighbors(p)
            .into_iter()
            .filter(|p| self.thing_at(*p).is_empty())
            .collect()
    }

    /// Return the creature that one at `ap` should attack, if any:
    /// the neighbor with
    /// the lowest hit points, and in the case of a tie the one first in reading
    /// order.
    pub fn target(&mut self, ap: Point) -> Option<Point> {
        let mut best_p: Option<Point> = None;
        let mut best_hp: usize = usize::max_value();
        let a_race = self.creature_at(ap).unwrap().race;
        for p in self.neighbors(ap).into_iter() {
            if let Some(pc) = self.creature_at(p) {
                if pc.race == a_race.enemy() && pc.hp < best_hp {
                    best_p = Some(p);
                    best_hp = pc.hp;
                }
            }
        }
        best_p
    }

    /// Hurt the creature at `tp` and
    /// maybe kill it. (It doesn't make any difference who's attacking it.)
    pub fn hurt(&mut self, tp: Point) {
        let mut target = self.creature_at_mut(tp).unwrap();
        if target.hp < ATTACK_POWER {
            println!("kill {:?}", target);
            self.set_thing_at(tp, Thing::Empty);
            self.cs.remove(&tp);
        } else {
            target.hp -= ATTACK_POWER;
        }
    }

    /// Return the creature at P, if any.
    pub fn creature_at_mut(&mut self, p: Point) -> Option<&mut Creature> {
        let th = self.thing_at(p);
        let r = self.cs.get_mut(&p);
        if let Some(ref c) = r {
            debug_assert_eq!(c.race, th);
        } else {
            debug_assert!(!th.is_creature());
        }
        r
    }

    pub fn creature_at(&self, p: Point) -> Option<&Creature> {
        let r = self.cs.get(&p);
        let th = self.thing_at(p);
        if let Some(ref c) = r {
            debug_assert_eq!(c.race, th);
        } else {
            debug_assert!(!th.is_creature());
        }
        r
    }

    pub fn thing_at(&self, p: Point) -> Thing {
        self.m[p]
    }

    fn set_thing_at(&mut self, p: Point, th: Thing) {
        self.m[p] = th
    }

    /// Return all the squares that are empty and neighbor creatures of the
    /// given race, in reading order.
    fn possible_move_targets(&self, race: Thing) -> Vec<Point> {
        let mut v: Vec<Point> = self
            .cs
            .values()
            .filter(|cr| cr.race == race)
            .flat_map(|cr| self.empty_neighbors(cr.p))
            .collect();
        // Although the creatures are visited in order, the points resulting
        // from them are not necessarily therefore in order, so sort.
        v.sort();
        v
    }
}

struct Routing {
    /// Distance from the origin.
    d: Matrix<Option<usize>>,

    /// Chosen destination
    chosen: Option<Point>,

    /// Next step to take from the origin: the lowest-ordered step towards
    /// the chosen destination.
    step: Option<Point>,
}

/// New approach on finding the best destination and first step towards it,
/// given a set of interesting destinations (from which you can attack enemies)
/// and a starting point.
///
/// Dests must be in sorted order.
impl Routing {
    pub fn new(m: &Map, origin: Point, dests: &[Point]) -> Routing {
        // All the points numbered in the last round, and needing to be
        // checked in the next round.
        let mut last = vec![origin];
        let mut d = Matrix::new(m.w, m.h, None);
        let mut dist = 0;
        let mut hit = false;
        d[origin] = Some(0);
        //
        // First, flood-fill out from the starting point. The starting point is
        // distance 0. Every empty immediate neighbor, not yet assigned a distance,
        // is distance 1. Etc.
        //
        // Do this by remembering all the squares that were numbered in the previous
        // round, and come back to visit all their empty, un-numbered neighbors.
        //
        // Stop after the complete round in which you assign a distance to any of
        // the possible destinations, or if we run out of reachable empty
        // squares to number.

        // TODO: We could skip pre-calculation of potential move targets, and
        // instead just wait to see when we arrive at a target of the given
        // race.

        // TODO: We could always visit potential destinations in point order,
        // rather than grouped around `lp`, and therefore not need to keep
        // a list of `dests` that are later filtered.

        while !hit && !last.is_empty() {
            let mut next = Vec::new();
            dist += 1;
            for lp in last.into_iter() {
                for np in m.empty_neighbors(lp).into_iter() {
                    if d[np].is_none() {
                        d[np] = Some(dist);
                        next.push(np);
                        if dests.contains(&np) {
                            hit = true;
                        }
                    }
                }
            }
            last = next;
        }

        // The first destination in order, for which we found a distance, is the
        // chosen one.
        //
        // But it might be that none of the destinations are actually reachable
        // from here.
        let chosen = dests.iter().filter(|p| d[**p].is_some()).next().cloned();
        if chosen.is_none() {
            return Routing {
                d,
                chosen,
                step: None,
            };
        }

        // Now walk back from that chosen point towards the origin, taking at each
        // step the smallest numbered point. Since we only ever numbered
        // the minimum number of squares to reach the destination, any path
        // we find must be a minimal path, and if we always prefer to go to
        // the smallest-numbered neighbors we must arrive at the smallest
        // destination.
        let mut j = dist;
        let mut backp = chosen.unwrap();
        'backup: while j > 1 {
            debug_assert_eq!(d[backp], Some(j));
            for np in m.empty_neighbors(backp) {
                if d[np] == Some(j - 1) {
                    j -= 1;
                    backp = np;
                    continue 'backup;
                }
            }
            panic!("No backup step found from {:?}", backp);
        }

        Routing {
            d,
            chosen,
            step: Some(backp),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2018::*;

    #[test]
    fn from_string() {
        let m = Map::from_string(
            "\
             #######\n\
             #.G.E.#\n\
             #E.G.E#\n\
             #.G.E.#\n\
             #######\n",
        );
        assert_eq!(m.cs.len(), 7);

        assert_eq!(
            m.neighbors(point(0, 0)),
            vec![Point { x: 1, y: 0 }, Point { x: 0, y: 1 }]
        );
    }

    #[test]
    fn target_example() {
        // This map has the default HP on everyone, so it doesn't influence
        // targeting.
        let mut m = Map::from_string(
            "\
             G....\n\
             ..G..\n\
             ..EG.\n\
             ..G..\n\
             ...G.\n",
        );
        // First goblin can't reach anything
        assert_eq!(m.target(point(0, 0)), None);
        // Second goblin should attack the elf.
        assert_eq!(
            m.creature_at(Point { x: 2, y: 1 }).unwrap().race,
            Thing::Goblin
        );
        assert_eq!(m.target(Point { x: 2, y: 1 }), Some(Point { x: 2, y: 2 }));
        // Elf should attack second goblin, because it's first in reading order.
        assert_eq!(m.target(Point { x: 2, y: 2 }), Some(Point { x: 2, y: 1 }));
    }

    #[test]
    fn combat_example() {
        let mut m = Map::from_string(
            "G....\n\
             ..G..\n\
             ..EG.\n\
             ..G..\n\
             ...G.\n",
        );
        // Tweak the HP to match the example
        m.creature_at_mut(point(0, 0)).unwrap().hp = 9;
        m.creature_at_mut(point(2, 1)).unwrap().hp = 4;
        m.creature_at_mut(point(3, 2)).unwrap().hp = 2;
        m.creature_at_mut(point(2, 3)).unwrap().hp = 2;
        m.creature_at_mut(point(3, 4)).unwrap().hp = 1;

        // Elf should attack second goblin, because it's the first in reading
        // order that has the lowest HP (2).
        let elf_p = point(2, 2);
        let tp = m.target(elf_p).unwrap();
        assert_eq!(tp, point(3, 2));

        // Actually attack it.
        m.hurt(tp);
        // Goblin on row 2 should have been killed.
        assert_eq!(m.thing_at(point(3, 2)), Thing::Empty);
        assert_eq!(m.creature_at(point(3, 2)), None);
        // Elf is still there.
        assert_eq!(m.creature_at(point(2, 2)).unwrap().race, Thing::Elf);
    }

    #[test]
    fn routing() {
        // Targets:      In range:     Reachable:    Nearest:      Chosen:
        // #######       #######       #######       #######       #######
        // #E..G.#       #E.?G?#       #E.@G.#       #E.!G.#       #E.+G.#
        // #...#.#  -->  #.?.#?#  -->  #.@.#.#  -->  #.!.#.#  -->  #...#.#
        // #.G.#G#       #?G?#G#       #@G@#G#       #!G.#G#       #.G.#G#
        // #######       #######       #######       #######       #######
        let m = Map::from_string(
            "\
             #######\n\
             #E..G.#\n\
             #...#.#\n\
             #.G.#G#\n\
             #######\n",
        );
        // Around the single elf, just two squares vacant.
        assert_eq!(
            m.possible_move_targets(Thing::Elf),
            vec![point(2, 1), point(1, 2)]
        );
        // Around the goblins, a few squares
        let pmt = m.possible_move_targets(Thing::Goblin);
        assert_eq!(
            pmt,
            vec![
                point(3, 1),
                point(5, 1),
                point(2, 2),
                point(5, 2),
                point(1, 3),
                point(3, 3)
            ]
        );
        let r = Routing::new(&m, point(1, 1), &pmt);

        assert_eq!(r.d[point(1, 1)], Some(0)); // the elf itself
        assert_eq!(r.d[point(2, 1)], Some(1));
        assert_eq!(r.d[point(3, 1)], Some(2));

        assert_eq!(r.d[point(1, 2)], Some(1));
        assert_eq!(r.d[point(2, 2)], Some(2));
        // This one shouldn't be calculated because it requires going further
        // than the first destination.
        assert_eq!(r.d[point(3, 2)], None);

        assert_eq!(r.d[point(1, 3)], Some(2));

        // The lowest-point destination is chosen.
        assert_eq!(r.chosen, Some(point(3, 1)));

        // The first step is towards it.
        assert_eq!(r.step, Some(point(2, 1)));
    }

    /// An example of routing where there are multiple paths
    #[test]
    fn routing2() {
        let m = Map::from_string(
            "\
             #######\n\
             #.E...#\n\
             #.....#\n\
             #...G.#\n\
             #######\n",
        );
        let pmt = m.possible_move_targets(Thing::Goblin);
        assert_eq!(pmt, vec![point(4, 2), point(3, 3), point(5, 3)]);

        let r = Routing::new(&m, point(2, 1), &pmt);
        assert_eq!(r.chosen, Some(point(4, 2)));
        assert_eq!(r.step, Some(point(3, 1)));
    }
}
