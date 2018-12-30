#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

/// https://adventofcode.com/2018/day/15
use aoc2018::matrix::Matrix;
use aoc2018::{point, Point};

const INITIAL_HP: usize = 200;
const ATTACK_POWER: usize = 3;

pub fn main() {
    let mut s = String::new();
    File::open("input/input15.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let mut m = Map::from_string(&s);
    println!("Result: {:?}", m.battle());
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

    pub fn to_char(self) -> char {
        match self {
            Thing::Empty => '.',
            Thing::Wall => '#',
            Thing::Elf => 'E',
            Thing::Goblin => 'G',
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
    completed_rounds: usize,
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
            completed_rounds: 0,
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
    pub fn target(&mut self, ap: Point, a_race: Thing) -> Option<Point> {
        let mut best_p: Option<Point> = None;
        let mut best_hp: usize = usize::max_value();
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
        if target.hp <= ATTACK_POWER {
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

    fn move_creature(&mut self, oldp: Point, newp: Point) {
        debug_assert!(self.thing_at(newp).is_empty());
        let mut c = self.cs.remove(&oldp).unwrap();
        debug_assert!(c.race.is_creature());
        debug_assert_eq!(c.p, oldp);
        debug_assert!(c.hp > 0);
        c.p = newp;
        self.set_thing_at(newp, c.race);
        self.set_thing_at(oldp, Thing::Empty);
        self.cs.insert(newp, c);
    }

    pub fn render(&self) -> String {
        let mut s = format!("Round: {}\n", self.completed_rounds);
        for y in 0..self.h {
            for x in 0..self.w {
                let p = point(x, y);
                s.push(self.thing_at(p).to_char());
            }
            s.push('\n');
        }
        s
    }

    pub fn any_creatures(&self, race: Thing) -> bool {
        self.cs.values().any(|c| c.race == race)
    }

    /// One round of the simulation.
    ///
    /// Returns false if, before the end of the round, any creature finds
    /// that there are no more enemy creatures anywhere, and so the game is
    /// over.
    pub fn round(&mut self) -> bool {
        // For all creatures, in reading order:
        //
        // If the creature was killed since the round started, skip it.
        //
        // If they are not currently next to an enemy, plot a route and move
        // towards one.
        //
        // If they are now next to an enemy, hurt it.
        let cps: Vec<Point> = self.cs.keys().cloned().collect();
        for mut cp in cps.into_iter() {
            let th = self.thing_at(cp);
            if !th.is_creature() {
                continue; // maybe already killed
            }
            if !self.any_creatures(th.enemy()) {
                return false;
            }
            if self.target(cp, th).is_none() {
                if let Some(r) = Routing::new(&self, cp, th.enemy()) {
                    println!(
                        "move {:?} from {:?} to {:?} towards {:?}, {} steps",
                        th, cp, r.step, r.chosen, r.dist
                    );
                    let newp = r.step;
                    self.move_creature(cp, newp);
                    cp = newp;
                }
            };
            if let Some(tp) = self.target(cp, th) {
                self.hurt(tp);
            }
        }
        self.completed_rounds += 1;
        true
    }

    // Play the whole battle until the end.
    // Returns: number of *completed* rounds, total HP of survivors, and the
    // product of the two.
    pub fn battle(&mut self) -> (usize, usize, usize) {
        while self.round() {
            print!("{}", self.render());
        }
        let remain_hp: usize = self.cs.values().map(|c| c.hp).sum();
        (
            self.completed_rounds,
            remain_hp,
            self.completed_rounds * remain_hp,
        )
    }
}

struct Routing {
    /// Chosen destination
    chosen: Point,

    /// Next step to take from the origin: the lowest-ordered step towards
    /// the chosen destination.
    step: Point,

    /// Distance to the chosen destination
    dist: usize,
}

/// New approach on finding the best destination and first step towards it,
/// given a set of interesting destinations (from which you can attack enemies)
/// and a starting point.
///
/// Dests must be in sorted order.
impl Routing {
    pub fn new(m: &Map, origin: Point, enemy: Thing) -> Option<Routing> {
        // All the points numbered in the last round, and needing to be
        // checked in the next round.
        let mut last = vec![origin];
        let mut d = Matrix::new(m.w, m.h, None);
        let mut dist = 0;
        // Points neighboring an enemy.
        let mut ends = Vec::new();
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

        // TODO: We could always visit potential destinations in point order,
        // rather than grouped around `lp`, and therefore not need to keep
        // a list of `dests` that are later filtered.

        println!("routing from {:?} looking for {:?}", origin, enemy);

        while ends.is_empty() && !last.is_empty() {
            let mut next = Vec::new();
            dist += 1;
            for lp in last.into_iter() {
                for np in m.neighbors(lp).into_iter() {
                    if m.thing_at(np) == enemy {
                        // lp neighbors an enemy; we could stop here.
                        println!("found enemy at {:?} from {:?} after {:?}", np, lp, dist);
                        ends.push(lp);
                    } else if m.thing_at(np).is_empty() && d[np].is_none() {
                        // We could move to np along this path; let's see if
                        // we could move further
                        d[np] = Some(dist);
                        next.push(np);
                    }
                }
            }
            last = next;
        }

        // Chosen destination is the first one, in order.
        if ends.is_empty() {
            // Filled as much of the map as we can, without finding any
            // reachable enemies. Let's stop.
            return None;
        };
        let chosen = *ends.iter().min().unwrap();

        // Now walk back from that chosen point towards the origin, taking at each
        // step the smallest numbered point. Since we only ever numbered
        // the minimum number of squares to reach the destination, any path
        // we find must be a minimal path, and if we always prefer to go to
        // the smallest-numbered neighbors we must arrive at the smallest
        // destination.
        let mut j = dist - 1;
        let mut backp = chosen;
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

        Some(Routing {
            chosen,
            step: backp,
            dist,
        })
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
        assert_eq!(m.target(point(0, 0), Thing::Goblin), None);
        // Second goblin should attack the elf.
        assert_eq!(
            m.creature_at(Point { x: 2, y: 1 }).unwrap().race,
            Thing::Goblin
        );
        assert_eq!(
            m.target(Point { x: 2, y: 1 }, Thing::Goblin),
            Some(Point { x: 2, y: 2 })
        );
        // Elf should attack second goblin, because it's first in reading order.
        assert_eq!(
            m.target(Point { x: 2, y: 2 }, Thing::Elf),
            Some(Point { x: 2, y: 1 })
        );
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
        let tp = m.target(elf_p, Thing::Elf).unwrap();
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
        let r = Routing::new(&m, point(1, 1), Thing::Goblin).unwrap();

        // The lowest-point destination is chosen.
        assert_eq!(r.chosen, point(3, 1));

        // The first step is towards it.
        assert_eq!(r.step, point(2, 1));
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
        let r = Routing::new(&m, point(2, 1), Thing::Goblin).unwrap();
        assert_eq!(r.chosen, point(4, 2));
        assert_eq!(r.step, point(3, 1));
    }

    #[test]
    fn full_example() {
        let mut m = Map::from_string(
            "\
             #######\n\
             #.G...#\n\
             #...EG#\n\
             #.#.#G#\n\
             #..G#E#\n\
             #.....#\n\
             #######\n\
             ",
        );
        for _ in 0..3 {
            print!("{}", m.render());
            m.round();
        }
        for _ in 3..23 {
            m.round();
        }
        for _ in 23..=28 {
            print!("{}", m.render());
            m.round();
        }
        for _ in 29..46 {
            assert!(m.round());
        }
        print!("{}", m.render());
        assert!(m.round()); // 46
        print!("{}", m.render());
        assert!(!m.round()); // 47, the end
        print!("{}", m.render());
        assert!(!m.round()); // 48, nothing else changes
    }

    #[test]
    fn battle() {
        let mut m = Map::from_string(
            "\
             #######\n\
             #.G...#\n\
             #...EG#\n\
             #.#.#G#\n\
             #..G#E#\n\
             #.....#\n\
             #######\n\
             ",
        );
        assert_eq!(m.battle(), (47, 590, 27730));
    }

    #[test]
    fn battle2() {
        let mut m = Map::from_string(
            "\
             #######\n\
             #G..#E#\n\
             #E#E.E#\n\
             #G.##.#\n\
             #...#E#\n\
             #...E.#\n\
             #######\n\
             ",
        );
        assert_eq!(m.battle(), (37, 982, 36334));
    }

    #[test]
    fn battle3() {
        let mut m = Map::from_string(
            "\
             #######\n\
             #.E...#\n\
             #.#..G#\n\
             #.###.#\n\
             #E#G#G#\n\
             #...#G#\n\
             #######\n\
             ",
        );
        assert_eq!(m.battle(), (54, 536, 28944));
    }

    #[test]
    fn battle4() {
        let mut m = Map::from_string(
            "\
             #########\n\
             #G......#\n\
             #.E.#...#\n\
             #..##..G#\n\
             #...##..#\n\
             #...#...#\n\
             #.G...G.#\n\
             #.....G.#\n\
             #########\n\
             ",
        );
        assert_eq!(m.battle(), (20, 937, 18740));
    }
}
