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

// #![allow(dead_code)]

//! https://adventofcode.com/2018/day/24
//!
//! An iterative battle between units on two sides.

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "aoc24.pest"]
pub struct AoC24Parser;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Attack {
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
    Slashing,
}
use Attack::*;

impl std::str::FromStr for Attack {
    type Err = AttackParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bludgeoning" => Ok(Bludgeoning),
            "cold" => Ok(Cold),
            "fire" => Ok(Fire),
            "radiation" => Ok(Radiation),
            "slashing" => Ok(Slashing),
            _ => Err(AttackParseError {}),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Side {
    Immune,
    Infection,
}
use Side::*;

#[derive(Debug, Clone, Copy)]
struct AttackParseError {}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Group {
    // Number of remaining units
    n_units: usize,

    // HP per unit
    hp: usize,

    // Double damage from these attacks
    weaknesses: Vec<Attack>,

    // Zero damage from these attacks
    immunities: Vec<Attack>,

    damage: usize,
    attack: Attack,

    initiative: usize,
    side: Side,
}

impl Group {
    /// Each group also has an effective power: the number of units
    /// in that group multiplied by their attack damage.
    pub fn power(&self) -> usize {
        self.n_units * self.damage
    }

    /// Take damage to a group, eliminating all the units that are reduced to
    /// zero, and ignoring any leftover damage.
    ///
    /// Returns true if at least one unit was eliminated.
    fn take_damage(&mut self, damage: usize) -> bool {
        let killed_units = damage / self.hp;
        self.n_units = self.n_units.saturating_sub(killed_units);
        killed_units > 0
    }

    fn alive(&self) -> bool {
        self.n_units > 0
    }
}

type GroupId = usize;

/// Calculate the potential damage that an attacker could do to a target.
///
/// This is "after accounting for weaknesses and immunities, but not accounting
/// for whether the defending group has enough units to actually receive
/// all of that damage", as is needed for choosing targets.
fn potential_damage(attacker: &Group, target: &Group) -> usize {
    if target.immunities.contains(&attacker.attack) {
        0
    } else if target.weaknesses.contains(&attacker.attack) {
        2 * attacker.power()
    } else {
        attacker.power()
    }
}

fn can_damage(attacker: &Group, target: &Group) -> bool {
    potential_damage(attacker, target) > 0
}

/// Given a pest iterator that contains one num, return the parsed num.
fn get_num(mut pairs: pest::iterators::Pairs<'_, Rule>) -> usize {
    pairs
        .find(|p| p.as_rule() == Rule::num)
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

/// Return the indexes of groups that are still alive.
fn live_units(gs: &[Group]) -> Vec<GroupId> {
    gs.iter()
        .enumerate()
        .filter(|(_i, g)| g.alive())
        .map(|(i, _)| i)
        .collect()
}

/// Return a vec of bools that is true  where the corresponding unit is alive.
fn live_unit_mask(gs: &[Group]) -> Vec<bool> {
    gs.iter().map(|g| g.alive()).collect()
}

#[cfg(test)]
fn summarize_state(gs: &[Group]) -> String {
    fn print_list<'a, W: std::fmt::Write, I: Iterator<Item = &'a Group>>(w: &mut W, it: I) {
        let mut it = it.enumerate().filter(|(_i, g)| g.alive()).peekable();
        if it.peek().is_none() {
            writeln!(w, "No groups remain.").unwrap();
            return;
        }
        for (i, g) in it {
            writeln!(w, "Group {} contains {} units", i + 1, g.n_units).unwrap();
        }
    }

    let mut s = String::new();
    s.push_str("Immune System:\n");
    print_list(&mut s, gs.iter().filter(|&g| g.side == Immune));
    s.push_str("Infection:\n");
    print_list(&mut s, gs.iter().filter(|&g| g.side == Infection));
    s
}

/// Sort a list of groups into the order in which they select targets.
///
/// Returns a list of indexes into the array that contains indexes for all live units and no
/// duplicates.
fn target_selection_order(gs: &[Group]) -> Vec<GroupId> {
    let mut gids = live_units(gs);
    // In decreasing order of effective power, groups choose their
    // targets; in a tie, the group with the higher initiative chooses first.
    gids.sort_by(|&i, &j| {
        let a = &gs[i];
        let b = &gs[j];
        b.power()
            .cmp(&a.power())
            .then(b.initiative.cmp(&a.initiative))
    });
    gids
}

/// Return vec of group ids in the order in which they will attack.
///
/// Note this is different to the order in which they select targets.
///
/// Groups attack in decreasing order of initiative, regardless of whether they are part of the
/// infection or the immune system. (If a group contains no units, it cannot attack.)
///
/// Note also that targets might be eliminated during the round, after this calculation.
fn attack_order(gs: &[Group]) -> Vec<GroupId> {
    let mut gids = live_units(gs);
    gids.sort_by(|&i, &j| gs[i].initiative.cmp(&gs[j].initiative).reverse());
    gids
}

/// For one attacker, choose the target it will attack.
fn choose_target(gs: &[Group], attacker_id: GroupId, target_mask: &[bool]) -> Option<GroupId> {
    // The attacking group chooses to target the group in the enemy army to
    // which it would deal the most damage (after accounting for weaknesses
    // and immunities, but not accounting for whether the defending group has
    // enough units to actually receive all of that damage).

    // If an attacking group is considering two defending groups to which it
    // would deal equal damage, it chooses to target the defending group with
    // the largest effective power; if there is still a tie, it chooses the
    // defending group with the highest initiative. If it cannot deal any
    // defending groups damage, it does not choose a target. Defending groups
    // can only be chosen as a target by one attacking group.

    // The `filter` lets us first discard indexes that can't be damaged, so
    // the `max_by_key` will see an empty input and return None.
    let attacker = &gs[attacker_id];
    gs.iter()
        .enumerate()
        .filter(|(i, g)| g.side != attacker.side && can_damage(attacker, g) && target_mask[*i])
        .max_by_key(|(_i, g)| (potential_damage(attacker, g), g.power(), g.initiative))
        .map(|(i, _g)| i)
}

/// Choose units to be attacked by all surving units.
///
/// Returns a vec t where t[i] is the target (if any) of unit i.
fn select_targets(gs: &[Group]) -> Vec<Option<GroupId>> {
    let tso = target_selection_order(gs);
    let mut targs = vec![None; gs.len()];
    let mut mask = live_unit_mask(gs);
    for attacker_id in tso {
        if let Some(target_id) = choose_target(gs, attacker_id, &mask) {
            // println!("{} selects target {}", attacker_id, target_id);
            debug_assert_eq!(targs[attacker_id], None);
            debug_assert!(!targs.contains(&Some(target_id)));
            targs[attacker_id] = Some(target_id);
            mask[target_id] = false;
        }
    }
    targs
}

/// One round of attacks across all groups that are alive and able to attack.
///
/// Some input values can cause a battle to a draw, where there are units remaining for both
/// sides but neither can do enough damage to eliminate even a single opposing unit.
/// If that happens, this returns false.
fn attack_round(gs: &mut [Group]) -> bool {
    let targs = select_targets(gs);
    let mut progress = false;
    for attacker_id in attack_order(gs) {
        if let Some(target_id) = targs[attacker_id] {
            if !gs[attacker_id].alive() || !gs[target_id].alive() {
                continue;
            }
            let dam = potential_damage(&gs[attacker_id], &gs[target_id]);
            // println!("{} attacks {} for {} damage", attacker_id, target_id, dam);
            progress |= gs[target_id].take_damage(dam);
        }
    }
    progress
}

/// Repeat attack rounds until someone wins, and return the winner and their number
/// of remaining units.
///
/// Returns None if the battle gets stuck in a stalemate.
fn attack_repeatedly(gs: &mut [Group]) -> Option<(Side, usize)> {
    loop {
        if !attack_round(gs) {
            return None;
        }
        if let Some((s, v)) = victory_condition(&gs) {
            return Some((s, v));
        }
    }
}

/// If there is only one side remaining, return which side won and
/// the total number of units it has.
fn victory_condition(gs: &[Group]) -> Option<(Side, usize)> {
    let mut n_immune = 0;
    let mut n_infection = 0;
    for g in gs {
        match g.side {
            Immune => n_immune += g.n_units,
            Infection => n_infection += g.n_units,
        }
    }
    if n_immune == 0 {
        assert!(n_infection > 0);
        Some((Infection, n_infection))
    } else if n_infection == 0 {
        Some((Immune, n_immune))
    } else {
        None
    }
}

fn parse_groups(pairs: pest::iterators::Pairs<'_, Rule>, side: Side, r: &mut Vec<Group>) {
    for ig in pairs {
        assert_eq!(ig.as_rule(), Rule::group);

        let mut n_units: Option<usize> = None;
        let mut hp: Option<usize> = None;
        let mut initiative: Option<usize> = None;
        let mut damage: Option<usize> = None;
        let mut attack: Option<Attack> = None;
        let mut weaknesses: Vec<Attack> = Vec::new();
        let mut immunities: Vec<Attack> = Vec::new();

        for i in ig.into_inner() {
            match i.as_rule() {
                Rule::units => n_units = Some(get_num(i.into_inner())),
                Rule::hp => hp = Some(get_num(i.into_inner())),
                Rule::initiative => initiative = Some(get_num(i.into_inner())),
                Rule::attack => {
                    for j in i.into_inner() {
                        match j.as_rule() {
                            Rule::num => damage = Some(j.as_str().parse().unwrap()),
                            Rule::weapon => attack = Some(j.as_str().parse().unwrap()),
                            other => panic!("unexpected {:#?}", other),
                        }
                    }
                }
                Rule::vuln => {
                    for j in i.into_inner() {
                        let r = j.as_rule();
                        let weps = j
                            .into_inner()
                            .map(|f| f.as_str().parse().unwrap())
                            .collect();
                        match r {
                            Rule::weaknesses => weaknesses = weps,
                            Rule::immunities => immunities = weps,
                            other => panic!("unexpected {:#?}", other),
                        }
                    }
                }
                other => panic!("unexpected {:#?}", other),
            }
        }

        r.push(Group {
            n_units: n_units.unwrap(),
            hp: hp.unwrap(),
            initiative: initiative.unwrap(),
            damage: damage.unwrap(),
            attack: attack.unwrap(),
            weaknesses,
            immunities,
            side,
        });
    }
    // The attack ordering algorithm won't be stable if there are any duplicate initiative values,
    // so let's make sure there aren't.
    let mut seen_initiative = Vec::new();
    for g in r {
        assert!(!seen_initiative.contains(&g.initiative));
        seen_initiative.push(g.initiative);
    }
}

fn load_input() -> Vec<Group> {
    parse_string(&std::fs::read_to_string("input/input24.txt").unwrap())
}

fn parse_string(s: &str) -> Vec<Group> {
    let f = AoC24Parser::parse(Rule::file, &s)
        .expect("failed to parse")
        .next()
        .unwrap();
    let mut gs: Vec<Group> = Vec::new();
    for i in f.into_inner() {
        match i.as_rule() {
            Rule::immune_system => parse_groups(i.into_inner(), Immune, &mut gs),
            Rule::infection => parse_groups(i.into_inner(), Infection, &mut gs),
            Rule::EOI => (),
            other => panic!("unexpected {:#?}", other),
        }
    }
    gs
}

/// Give a boost to the damage of all units on one side.
fn boost_side_damage(gs: &mut [Group], side: Side, boost: usize) {
    for g in gs.iter_mut() {
        if g.side == side {
            g.damage += boost
        }
    }
}

fn solve_a() -> usize {
    attack_repeatedly(&mut load_input()).unwrap().1
}

fn solve_b() -> usize {
    let orig_gs = load_input();

    let boosted_battle = |boost| {
        let mut gs = orig_gs.clone();
        boost_side_damage(&mut gs, Immune, boost);
        attack_repeatedly(&mut gs)
    };

    let best_boost = aoc2018::bisection_search(0, 1_000_000, |boost| {
        match boosted_battle(boost) {
            // In a stalemate, assume we need to increase the boost.
            None => false,
            Some((Immune, _)) => true,
            Some((Infection, _)) => false,
        }
    })
    .unwrap();
    // Now recompute the results for that best battle
    boosted_battle(best_boost).unwrap().1
}

pub fn main() {
    println!("Solution A: {}", solve_a());
    println!("Solution B: {}", solve_b());
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str =
            "\
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
 ";

    #[test]
    fn known_solution_a() {
        assert_eq!(solve_a(), 22996);
    }

    #[test]
    fn known_solution_b() {
        assert_eq!(solve_b(), 4327);
    }

    #[test]
    fn load_input() {
        let gs = super::load_input();
        assert_eq!(gs.len(), 20);
        assert_eq!(gs.iter().filter(|g| g.side == Immune).count(), 10);
        assert_eq!(gs.iter().filter(|g| g.side == Infection).count(), 10);
        assert_eq!(
            gs[9],
            Group {
                n_units: 742,
                hp: 1702,
                weaknesses: vec![Radiation],
                immunities: vec![Slashing],
                damage: 22,
                attack: Radiation,
                initiative: 13,
                side: Immune,
            }
        );

        let _targs = select_targets(&gs);
    }

    #[test]
    fn example_a() {
        let mut gs = super::parse_string(EXAMPLE);
        assert_eq!(gs.len(), 4);
        assert_eq!(
            gs[0],
            Group {
                n_units: 17,
                hp: 5390,
                weaknesses: vec![Radiation, Bludgeoning],
                immunities: vec![],
                attack: Fire,
                damage: 4507,
                initiative: 2,
                side: Immune
            }
        );

        // All of them are live.
        assert_eq!(live_units(&gs), vec![0, 1, 2, 3]);

        let tso = super::target_selection_order(&gs);
        // Target selection proceeds in order of decreasing power (units * damage).
        assert_eq!(tso, vec![2, 0, 3, 1]);

        let targs = select_targets(&gs);
        assert_eq!(targs, vec![Some(3), Some(2), Some(0), Some(1)]);

        assert_eq!(attack_order(&gs), vec![3, 1, 0, 2]);

        println!("{}", summarize_state(&gs));
        assert_eq!(victory_condition(&gs), None);

        assert_eq!(
            summarize_state(&gs),
            "\
Immune System:
Group 1 contains 17 units
Group 2 contains 989 units
Infection:
Group 1 contains 801 units
Group 2 contains 4485 units
"
        );

        attack_round(&mut gs);
        println!("{}", summarize_state(&gs));
        assert_eq!(
            summarize_state(&gs),
            "\
Immune System:
Group 2 contains 905 units
Infection:
Group 1 contains 797 units
Group 2 contains 4434 units
"
        );
        assert_eq!(victory_condition(&gs), None);

        attack_round(&mut gs);
        println!("{}", summarize_state(&gs));
        assert_eq!(
            summarize_state(&gs),
            "\
Immune System:
Group 2 contains 761 units
Infection:
Group 1 contains 793 units
Group 2 contains 4434 units
"
        );
        assert_eq!(victory_condition(&gs), None);

        attack_round(&mut gs);
        println!("{}", summarize_state(&gs));
        assert_eq!(
            summarize_state(&gs),
            "\
Immune System:
Group 2 contains 618 units
Infection:
Group 1 contains 789 units
Group 2 contains 4434 units
"
        );
        assert_eq!(victory_condition(&gs), None);

        attack_round(&mut gs);
        assert_eq!(
            summarize_state(&gs),
            "\
Immune System:
Group 2 contains 475 units
Infection:
Group 1 contains 786 units
Group 2 contains 4434 units
"
        );
        assert_eq!(victory_condition(&gs), None);

        attack_round(&mut gs);
        assert_eq!(
            summarize_state(&gs),
            "\
Immune System:
Group 2 contains 333 units
Infection:
Group 1 contains 784 units
Group 2 contains 4434 units
"
        );
        assert_eq!(victory_condition(&gs), None);

        attack_round(&mut gs);
        assert_eq!(
            summarize_state(&gs),
            "\
Immune System:
Group 2 contains 191 units
Infection:
Group 1 contains 783 units
Group 2 contains 4434 units
"
        );
        assert_eq!(victory_condition(&gs), None);

        attack_round(&mut gs);
        assert_eq!(
            summarize_state(&gs),
            "\
Immune System:
Group 2 contains 49 units
Infection:
Group 1 contains 782 units
Group 2 contains 4434 units
"
        );
        assert_eq!(victory_condition(&gs), None);

        attack_round(&mut gs);
        assert_eq!(
            summarize_state(&gs),
            "\
Immune System:
No groups remain.
Infection:
Group 1 contains 782 units
Group 2 contains 4434 units
"
        );
        assert_eq!(victory_condition(&gs), Some((Infection, 5216)));
    }

    #[test]
    fn example_b() {
        let mut gs = super::parse_string(EXAMPLE);
        boost_side_damage(&mut gs, Immune, 1570);
        let (s, v) = attack_repeatedly(&mut gs).unwrap();
        assert_eq!((s, v), (Immune, 51));
    }
}
