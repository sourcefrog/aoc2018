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
    fn take_damage(&mut self, damage: usize) {
        self.n_units = self.n_units.saturating_sub(damage / self.hp);
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
fn choose_target(gs: &[Group], attacker_id: GroupId, target_ids: &[GroupId]) -> Option<GroupId> {
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
    target_ids
        .iter()
        .filter(|&&i| gs[i].side != attacker.side && can_damage(attacker, &gs[i]))
        .max_by_key(|&&i| {
            let t = &gs[i];
            (potential_damage(attacker, t), t.power(), t.initiative)
        })
        .cloned()
}

/// Choose units to be attacked by all surving units.
///
/// Returns a vec of (attacker, target) indicating selections.
fn select_targets(gs: &[Group]) -> Vec<(GroupId, GroupId)> {
    let tso = target_selection_order(gs);
    let mut r = Vec::new();
    // Each target can only be selected by one attacker.
    let mut remaining_targets = live_units(gs);
    for attacker_id in tso {
        debug_assert!(
            r.iter().all(|&(a, _t)| a != attacker_id),
            "attacker occurs twice"
        );
        if let Some(target_id) = choose_target(gs, attacker_id, &remaining_targets) {
            // println!("{} selects target {}", attacker_id, target_id);
            remaining_targets.retain(|&i| i != target_id);
            debug_assert!(
                r.iter().all(|&(_a, t)| t != target_id),
                "target selected twice"
            );
            r.push((attacker_id, target_id));
        } else {
            // println!("{} found no target", attacker_id);
        }
    }
    // println!( "these groups are targeted by nobody: {:?}", remaining_targets);
    r
}

/// One round of attacks across all groups that are alive and able to attack.
fn attack_round(gs: &mut [Group]) {
    let targs = select_targets(gs);
    for attacker_id in attack_order(gs) {
        if !gs[attacker_id].alive() {
            continue;
        }
        if let Some(target_id) = targs
            .iter()
            .find(|(a, _t)| *a == attacker_id)
            .map(|(_a, t)| *t)
        {
            // println!("{} attacks {}", attacker_id, target_id);
            if !gs[target_id].alive() {
                // println!("   ... but it's already dead");
                continue;
            }
            let dam = potential_damage(&gs[attacker_id], &gs[target_id]);
            gs[target_id].take_damage(dam);
        }
    }
}

/// If there is only one side remaining, return the total number of units it
/// has.
fn victory_condition(gs: &[Group]) -> Option<usize> {
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
        Some(n_infection)
    } else if n_infection == 0 {
        Some(n_immune)
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

fn solve_a() -> usize {
    let mut gs = load_input();
    loop {
        attack_round(&mut gs);
        if let Some(v) = victory_condition(&gs) {
            return v;
        }
    }
}

pub fn main() {
    println!("Solution A: {}", solve_a());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn known_solution_a() {
        assert_eq!(solve_a(), 22996);
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
    fn example() {
        let mut gs = super::parse_string(
            "\
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
 ",
        );
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
        assert_eq!(targs, vec![(2, 0), (0, 3), (3, 1), (1, 2)]);

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
        assert_eq!(victory_condition(&gs), Some(5216));
    }
}
