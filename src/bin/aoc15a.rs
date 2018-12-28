#![allow(dead_code)]

use std::collections::BTreeMap;

/// https://adventofcode.com/2018/day/15
use aoc2018::matrix::Matrix;

const INITIAL_HP: usize = 200;
const ATTACK_POWER: usize = 3;

pub fn main() {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
struct Point {
    y: usize,
    x: usize,
}

/// Shorthand to construct a point.
fn point(x: usize, y: usize) -> Point {
    Point { x, y }
}

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
                if th == Thing::Elf || th == Thing::Goblin {
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
    pub fn neighbors(&self, p: &Point) -> Vec<Point> {
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
    pub fn empty_neighbors(&self, p: &Point) -> Vec<Point> {
        self.neighbors(p)
            .into_iter()
            .filter(|p| self.thing_at(p).is_empty())
            .collect()
    }

    /// Return the creature that one at `ap` should attack, if any:
    /// the neighbor with
    /// the lowest hit points, and in the case of a tie the one first in reading
    /// order.
    pub fn target(&mut self, ap: &Point) -> Option<Point> {
        let mut best_p: Option<Point> = None;
        let mut best_hp: usize = usize::max_value();
        let a_race = self.creature_at(ap).unwrap().race;
        for p in self.neighbors(ap).iter() {
            if let Some(pc) = self.creature_at(p) {
                if pc.race == a_race.enemy() && pc.hp < best_hp {
                    best_p = Some(*p);
                    best_hp = pc.hp;
                }
            }
        }
        best_p
    }

    /// Hurt the creature at `tp` and
    /// maybe kill it. (It doesn't make any difference who's attacking it.)
    pub fn hurt(&mut self, tp: &Point) {
        let mut target = self.creature_at_mut(tp).unwrap();
        assert!(target.race == Thing::Elf || target.race == Thing::Goblin);
        if target.hp < ATTACK_POWER {
            println!("kill {:?}", target);
            self.set_thing_at(tp, Thing::Empty);
            self.cs.remove(tp);
        } else {
            target.hp -= ATTACK_POWER;
        }
    }

    /// Return the creature at P, if any.
    pub fn creature_at_mut(&mut self, p: &Point) -> Option<&mut Creature> {
        let th = self.thing_at(p);
        let r = self.cs.get_mut(p);
        if let Some(ref c) = r {
            debug_assert_eq!(c.race, th);
        } else {
            debug_assert!(th != Thing::Elf && th != Thing::Goblin);
        }
        r
    }

    pub fn creature_at(&self, p: &Point) -> Option<&Creature> {
        let r = self.cs.get(p);
        let th = self.thing_at(p);
        if let Some(ref c) = r {
            debug_assert_eq!(c.race, th);
        } else {
            debug_assert!(th != Thing::Elf && th != Thing::Goblin);
        }
        r
    }

    pub fn thing_at(&self, p: &Point) -> Thing {
        self.m[(p.y, p.x)]
    }

    fn set_thing_at(&mut self, p: &Point, th: Thing) {
        self.m[(p.y, p.x)] = th
    }

    /// Return all the squares that are empty and neighbor creatures of the
    /// given race, in reading order.
    fn possible_move_targets(&self, race: Thing) -> Vec<Point> {
        let mut v: Vec<Point> = self
            .cs
            .values()
            .filter(|cr| cr.race == race)
            .flat_map(|cr| self.empty_neighbors(&cr.p))
            .collect();
        // Although the creatures are visited in order, the points resulting
        // from them are not necessarily therefore in order, so sort.
        v.sort();
        v
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
            m.neighbors(&Point { x: 0, y: 0 }),
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
        assert_eq!(m.target(&Point { x: 0, y: 0 }), None);
        // Second goblin should attack the elf.
        assert_eq!(
            m.creature_at(&Point { x: 2, y: 1 }).unwrap().race,
            Thing::Goblin
        );
        assert_eq!(m.target(&Point { x: 2, y: 1 }), Some(Point { x: 2, y: 2 }));
        // Elf should attack second goblin, because it's first in reading order.
        assert_eq!(m.target(&Point { x: 2, y: 2 }), Some(Point { x: 2, y: 1 }));
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
        m.creature_at_mut(&point(0, 0)).unwrap().hp = 9;
        m.creature_at_mut(&point(2, 1)).unwrap().hp = 4;
        m.creature_at_mut(&point(3, 2)).unwrap().hp = 2;
        m.creature_at_mut(&point(2, 3)).unwrap().hp = 2;
        m.creature_at_mut(&point(3, 4)).unwrap().hp = 1;

        // Elf should attack second goblin, because it's the first in reading
        // order that has the lowest HP (2).
        let elf_p = point(2, 2);
        let tp = m.target(&elf_p).unwrap();
        assert_eq!(tp, point(3, 2));

        // Actually attack it.
        m.hurt(&tp);
        // Goblin on row 2 should have been killed.
        assert_eq!(m.thing_at(&point(3, 2)), Thing::Empty);
        assert_eq!(m.creature_at(&point(3, 2)), None);
        // Elf is still there.
        assert_eq!(m.creature_at(&point(2, 2)).unwrap().race, Thing::Elf);
    }

    #[test]
    fn move_example() {
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
        assert_eq!(
            m.possible_move_targets(Thing::Goblin),
            vec![
                point(3, 1),
                point(5, 1),
                point(2, 2),
                point(5, 2),
                point(1, 3),
                point(3, 3)
            ]
        );
    }
}
