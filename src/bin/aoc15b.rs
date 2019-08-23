#![allow(dead_code)]

//! https://adventofcode.com/2018/day/15

use std::fs::File;
use std::io::Read;

use aoc2018::matrix::Matrix;
use aoc2018::{point, Point};

const INITIAL_HP: usize = 200;
const GOBLIN_POWER: usize = 3;

pub fn main() {
    let mut s = String::new();
    File::open("input/input15.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    for ep in 3..100 {
        let mut m = Map::from_string(&s);
        m.elf_power = ep;
        let initial_elves = m.n_elf;
        let battle_result = m.battle();
        println!(
            "Elf power={}, result: {:?}, final n_elf={}",
            m.elf_power, battle_result, m.n_elf
        );
        if m.n_elf == initial_elves {
            break;
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Thing {
    Empty,
    Wall,
    Elf(usize),
    Goblin(usize),
}

use self::Thing::*;

impl Thing {
    pub fn from_char(ch: char) -> Thing {
        match ch {
            '.' => Thing::Empty,
            '#' => Thing::Wall,
            'E' => Thing::Elf(INITIAL_HP),
            'G' => Thing::Goblin(INITIAL_HP),
            other => panic!("unexpected character {:?}", other),
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Thing::Empty => '.',
            Thing::Wall => '#',
            Thing::Elf(_) => 'E',
            Thing::Goblin(_) => 'G',
        }
    }

    /// The enemy race for creatures (only).
    pub fn is_enemy(&self, other: &Thing) -> bool {
        match (self, other) {
            (Thing::Goblin(_), Thing::Elf(_)) => true,
            (Thing::Elf(_), Thing::Goblin(_)) => true,
            _ => false,
        }
    }

    pub fn is_empty(self) -> bool {
        self == Thing::Empty
    }

    pub fn is_creature(&self) -> bool {
        match self {
            Thing::Elf(_) => true,
            Thing::Goblin(_) => true,
            _ => false,
        }
    }

    pub fn is_goblin(&self) -> bool {
        match self {
            Goblin(_) => true,
            _ => false,
        }
    }

    pub fn is_elf(&self) -> bool {
        match self {
            Elf(_) => true,
            _ => false,
        }
    }
    pub fn creature_hp(&self) -> Option<usize> {
        match self {
            Elf(hp) | Goblin(hp) => Some(*hp),
            _ => None,
        }
    }
}

struct Map {
    m: Matrix<Thing>,
    w: usize,
    h: usize,
    completed_rounds: usize,
    n_elf: usize,
    n_goblin: usize,
    pub elf_power: usize,
}

impl Map {
    pub fn from_string(s: &str) -> Map {
        let mut mb = Matrix::<Thing>::from_rows();
        let mut n_elf = 0;
        let mut n_goblin = 0;
        for l in s.lines() {
            let r: Vec<Thing> = l.chars().map(Thing::from_char).collect();
            mb.add_row(&r);
            for i in r {
                match i {
                    Elf(_) => {
                        n_elf += 1;
                    }
                    Goblin(_) => {
                        n_goblin += 1;
                    }
                    _ => (),
                }
            }
        }
        let m = mb.finish();
        Map {
            w: m.width(),
            h: m.height(),
            m,
            completed_rounds: 0,
            n_elf,
            n_goblin,
            elf_power: 3,
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
    pub fn target(&mut self, ap: Point, attacker: &Thing) -> Option<Point> {
        let mut best_p: Option<Point> = None;
        let mut best_hp: usize = usize::max_value();
        for p in self.neighbors(ap).into_iter() {
            let thingp = self.thing_at(p);
            if let Some(hp) = thingp.creature_hp() {
                if thingp.is_enemy(attacker) && hp < best_hp {
                    best_p = Some(p);
                    best_hp = hp
                }
            }
        }
        best_p
    }

    /// Hurt the creature at `tp` and
    /// maybe kill it. (It doesn't make any difference who's attacking it.)
    pub fn hurt(&mut self, tp: Point) {
        let old_thing = self.thing_at(tp);
        let hp = old_thing.creature_hp().unwrap();
        let damage = match old_thing {
            Elf(_) => GOBLIN_POWER,
            Goblin(_) => self.elf_power,
            _ => panic!(),
        };
        let new_thing = if hp <= damage {
            println!("kill {:?}", old_thing);
            if old_thing.is_elf() {
                self.n_elf -= 1;
            } else if old_thing.is_goblin() {
                self.n_goblin -= 1;
            }
            Empty
        } else {
            match old_thing {
                Elf(_) => Elf(hp - damage),
                Goblin(_) => Goblin(hp - damage),
                _ => panic!(),
            }
        };
        self.set_thing_at(tp, new_thing);
    }

    pub fn thing_at(&self, p: Point) -> Thing {
        self.m[p]
    }

    pub fn thing_at_mut(&mut self, p: Point) -> &mut Thing {
        &mut self.m[p]
    }

    fn set_thing_at(&mut self, p: Point, th: Thing) {
        self.m[p] = th
    }

    fn move_creature(&mut self, oldp: Point, newp: Point) {
        debug_assert!(self.thing_at(newp).is_empty());
        let actor = self.thing_at(oldp);
        debug_assert!(actor.is_creature());
        self.set_thing_at(newp, actor);
        self.set_thing_at(oldp, Thing::Empty);
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

    fn annihilated(&self) -> bool {
        self.n_elf == 0 || self.n_goblin == 0
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
        let mut recently_moved: Vec<Point> = Vec::new();
        for y in 0..self.h {
            for x in 0..self.w {
                let cp = point(x, y);
                let th = self.thing_at(cp);
                if !th.is_creature() {
                    continue;
                }
                if recently_moved.contains(&cp) {
                    // Creature moved into this square on this round; it doesn't
                    // get to go again.
                    continue;
                }
                if self.annihilated() {
                    // TODO: Maybe a bit slow to scan the whole map again, for every creature, on every move. We could keep a count per race.
                    return false;
                }
                if let Some(tp) = self.target(cp, &th) {
                    // Attack immediately without needing to move
                    self.hurt(tp);
                } else if let Some(r) = Routing::new(&self, cp) {
                    // Move, then try to attack.
                    // println!(
                    //     "move {:?} from {:?} to {:?} towards {:?}, {} steps",
                    //     th, cp, r.step, r.chosen, r.dist
                    // );
                    let newp = r.step;
                    self.move_creature(cp, newp);
                    recently_moved.push(newp);
                    if let Some(tp) = self.target(newp, &th) {
                        self.hurt(tp);
                    }
                }
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
            // print!("{}", self.render());
        }
        let mut remain_hp = 0;
        for y in 0..self.h {
            for x in 0..self.w {
                remain_hp += self.thing_at(point(x, y)).creature_hp().unwrap_or(0)
            }
        }
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
    pub fn new(m: &Map, origin: Point) -> Option<Routing> {
        // All the points numbered in the last round, and needing to be
        // checked in the next round.
        let mut last = vec![origin];
        let mut d = Matrix::new(m.w, m.h, None);
        let mut dist = 0;
        let actor = m.thing_at(origin);
        assert!(actor.is_creature());
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

        // println!("routing from {:?} at {:?}", actor, origin);

        while ends.is_empty() && !last.is_empty() {
            let mut next = Vec::new();
            dist += 1;
            for lp in last.into_iter() {
                for np in m.neighbors(lp).into_iter() {
                    if actor.is_enemy(&m.thing_at(np)) {
                        // lp neighbors an enemy; we could stop here.
                        // println!("found enemy at {:?} from {:?} after {:?}", np, lp, dist);
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
        assert_eq!(m.target(point(0, 0), &Goblin(200)), None);
        // Second goblin should attack the elf.
        assert!(m.thing_at(Point { x: 2, y: 1 }).is_goblin());
        assert_eq!(
            m.target(Point { x: 2, y: 1 }, &Goblin(200)),
            Some(Point { x: 2, y: 2 })
        );
        // Elf should attack second goblin, because it's first in reading order.
        assert_eq!(
            m.target(Point { x: 2, y: 2 }, &Elf(200)),
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
        m.set_thing_at(point(0, 0), Goblin(9));
        m.set_thing_at(point(2, 1), Goblin(4));
        m.set_thing_at(point(3, 2), Goblin(2));
        m.set_thing_at(point(2, 3), Goblin(2));
        m.set_thing_at(point(3, 4), Goblin(1));

        // Elf should attack second goblin, because it's the first in reading
        // order that has the lowest HP (2).
        let elf_p = point(2, 2);
        let tp = m.target(elf_p, &Elf(200)).unwrap();
        assert_eq!(tp, point(3, 2));

        // Actually attack it.
        m.hurt(tp);
        // Goblin on row 2 should have been killed.
        assert!(m.thing_at(point(3, 2)).is_empty());
        // Elf is still there.
        assert!(m.thing_at(point(2, 2)).is_elf());
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
        let r = Routing::new(&m, point(1, 1)).unwrap();

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
        let r = Routing::new(&m, point(2, 1)).unwrap();
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
