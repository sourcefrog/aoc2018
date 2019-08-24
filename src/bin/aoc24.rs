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
}

// Given a pest iterator that contains one num, return the parsed num.
fn get_num(mut pairs: pest::iterators::Pairs<'_, Rule>) -> usize {
    pairs
        .find(|p| p.as_rule() == Rule::num)
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

fn parse_groups(pairs: pest::iterators::Pairs<'_, Rule>) -> Vec<Group> {
    let mut r = Vec::new();

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
        });
    }
    r
}

fn load_input() -> (Vec<Group>, Vec<Group>) {
    let s = std::fs::read_to_string("input/input24.txt").unwrap();
    let f = AoC24Parser::parse(Rule::file, &s)
        .expect("failed to parse")
        .next()
        .unwrap();
    let mut immune_system: Vec<Group> = Vec::new();
    let mut infection: Vec<Group> = Vec::new();

    for i in f.into_inner() {
        match i.as_rule() {
            Rule::immune_system => immune_system = parse_groups(i.into_inner()),
            Rule::infection => infection = parse_groups(i.into_inner()),
            Rule::EOI => (),
            other => panic!("unexpected {:#?}", other),
        }
    }
    (immune_system, infection)
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
        let (immune_system, infection) = super::load_input();
        assert_eq!(immune_system.len(), 10);
        assert_eq!(
            immune_system[9],
            Group {
                n_units: 742,
                hp: 1702,
                weakness: vec![Radiation],
                immune: vec![Slashing],
                damage: 22,
                attack: Radiation,
                initiative: 13
            }
        );
        assert_eq!(infection.len(), 10);
    }
}
