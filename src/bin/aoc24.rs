#![allow(dead_code)]

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

#[derive(Debug, Clone, Copy)]
struct AttackParseError {}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Group {
    // Number of remaining units
    n_units: usize,

    // HP per unit
    hp: usize,

    // Double damage from these attacks
    weakness: Vec<Attack>,

    // Zero damage from these attacks
    immune: Vec<Attack>,

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
}

type GroupId = usize;

/// Calculate the potential damage that an attacker could do to a target.
///
/// This is "after accounting for weaknesses and immunities, but not accounting
/// for whether the defending group has enough units to actually receive
/// all of that damage", as is needed for choosing targets.
fn potential_damage(attacker: &Group, target: &Group) -> usize {
    if target.immune.contains(&attacker.attack) {
        0
    } else if target.weakness.contains(&attacker.attack) {
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
        .filter(|(_i, g)| g.n_units > 0)
        .map(|(i, _)| i)
        .collect()
}

/// Sort a list of groups into the order in which they attack.
///
/// Returns a list of indexes into the array.
fn sort_attackers(gs: &[Group]) -> Vec<GroupId> {
    let mut ix = live_units(gs);

    // In decreasing order of effective power, groups choose their
    // targets; in a tie, the group with the higher initiative chooses first.
    ix.sort_by(|i, j| {
        let a = &gs[*i];
        let b = &gs[*j];
        b.power()
            .cmp(&a.power())
            .then(b.initiative.cmp(&a.initiative))
    });
    ix
}

/// For one attacker, choose the target it will attack.
///
/// Targets are removed from the target vector as they're
fn choose_target(gs: &[Group], attacker_id: GroupId, targets: &[GroupId]) -> Option<GroupId> {
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
    targets
        .iter()
        .filter(|i| can_damage(attacker, &gs[**i]))
        .max_by_key(|i| {
            let t = &gs[**i];
            (potential_damage(attacker, t), t.power(), t.initiative)
        })
        .cloned()
}

fn parse_groups(pairs: pest::iterators::Pairs<'_, Rule>, side: Side, r: &mut Vec<Group>) {
    for ig in pairs {
        assert_eq!(ig.as_rule(), Rule::group);

        let mut n_units: Option<usize> = None;
        let mut hp: Option<usize> = None;
        let mut initiative: Option<usize> = None;
        let mut damage: Option<usize> = None;
        let mut attack: Option<Attack> = None;
        let mut weakness: Vec<Attack> = Vec::new();
        let mut immune: Vec<Attack> = Vec::new();

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
                            Rule::weakness => weakness = weps,
                            Rule::immune => immune = weps,
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
            weakness,
            immune,
            side,
        });
    }
}

fn load_input() -> Vec<Group> {
    let s = std::fs::read_to_string("input/input24.txt").unwrap();
    let f = AoC24Parser::parse(Rule::file, &s)
        .expect("failed to parse")
        .next()
        .unwrap();
    let mut gs: Vec<Group> = Vec::new();

    for i in f.into_inner() {
        match i.as_rule() {
            Rule::immune_system => parse_groups(i.into_inner(), Side::Immune, &mut gs),
            Rule::infection => parse_groups(i.into_inner(), Side::Infection, &mut gs),
            Rule::EOI => (),
            other => panic!("unexpected {:#?}", other),
        }
    }
    gs
}

pub fn main() {
    load_input();
}

#[cfg(test)]
mod test {
    #[allow(unused)]
    use super::*;

    #[test]
    fn load_input() {
        let gs = super::load_input();
        assert_eq!(gs.len(), 20);
        assert_eq!(gs.iter().filter(|g| g.side == Side::Immune).count(), 10);
        assert_eq!(gs.iter().filter(|g| g.side == Side::Infection).count(), 10);
        assert_eq!(
            gs[9],
            Group {
                n_units: 742,
                hp: 1702,
                weakness: vec![Radiation],
                immune: vec![Slashing],
                damage: 22,
                attack: Radiation,
                initiative: 13,
                side: Side::Immune,
            }
        );
    }
}
