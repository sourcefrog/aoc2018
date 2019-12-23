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

#![allow(dead_code)]

type Reg = usize;

/// An instruction
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Inst {
    /// addr (add register) stores into register C the result of adding register A and register B.
    Addr,
    /// addi (add immediate) stores into register C the result of adding register A and value B.
    Addi,
    /// mulr (multiply register) stores into register C the result of multiplying register A and register B.
    Mulr,
    /// muli (multiply immediate) stores into register C the result of multiplying register A and value B.
    Muli,
    /// banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
    Banr,
    /// bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
    Bani,
    /// borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
    Borr,
    /// bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
    Bori,
    /// setr (set register) copies the contents of register A into register C. (Input B is ignored.)
    Setr,
    /// seti (set immediate) stores value A into register C. (Input B is ignored.)
    Seti,
    /// gtir (greater-than immediate/register) sets register C to 1 if value A is greater than  register B. Otherwise, register C is set to 0.
    Gtir,
    /// gtri (greater-than register/immediate) sets register C to 1 if register A is greater than   value B. Otherwise, register C is set to 0.
    Gtri,
    /// gtrr (greater-than register/register) sets register C to 1 if register A is greater than    register B. Otherwise, register C is set to 0.
    Gtrr,
    /// eqir (equal immediate/register) sets register C to 1 if value A is equal to register B.     Otherwise, register C is set to 0.
    Eqir,
    /// eqri (equal register/immediate) sets register C to 1 if register A is equal to value B.     Otherwise,///  register C is set to 0.
    Eqri,
    /// eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
    Eqrr,
}
use self::Inst::*;

impl Inst {
    pub fn apply(self, r: &[Reg; 4], args: &[Reg; 4]) -> [Reg; 4] {
        let [_, a, b, c] = *args;
        let mut after = *r;
        // Happily in Rust if you cast a bool to an int, it goes to 1 and 0.
        after[c] = match self {
            Muli => r[a] * b,
            Mulr => r[a] * r[b],
            Addr => r[a] + r[b],
            Addi => r[a] + b,
            Banr => r[a] & r[b],
            Bani => r[a] & b,
            Borr => r[a] | r[b],
            Bori => r[a] | b,
            Setr => r[a],
            Seti => a,
            Gtir => (a > r[b]) as Reg,
            Gtri => (r[a] > b) as Reg,
            Gtrr => (r[a] > r[b]) as Reg,
            Eqir => (a == r[b]) as Reg,
            Eqri => (r[a] == b) as Reg,
            Eqrr => (r[a] == r[b]) as Reg,
        };
        after
    }

    pub fn all() -> &'static [Inst] {
        &[
            Muli, Mulr, Addr, Addi, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir,
            Eqri, Eqrr,
        ]
    }
}

fn parse_number_list(s: &str, sep: &str) -> Vec<Reg> {
    let mut v = Vec::with_capacity(4);
    for a in s.split(sep) {
        v.push(a.parse().unwrap());
    }
    v
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Sample {
    /// Register values before the operation.
    before: [Reg; 4],
    /// Encoded instruction, consisting of an opcode followed by three arguments.
    ops: [Reg; 4],
    /// Regesters afterwards.
    after: [Reg; 4],
}

impl Sample {
    pub fn parse_samples<S: AsRef<str>>(l: &[S]) -> Vec<Sample> {
        let mut v = Vec::new();
        let mut it = l.iter();
        while let Some(l1) = it.next() {
            let l1 = l1.as_ref();
            if l1.is_empty() {
                break;
            }
            assert!(l1.starts_with("Before: ["));
            assert!(l1.ends_with(']'));
            let before = parse_number_list(&l1[9..(l1.len() - 1)], ", ");
            assert_eq!(before.len(), 4);

            let l2 = it.next().unwrap().as_ref();
            let ops = parse_number_list(l2, " ");
            assert_eq!(ops.len(), 4);

            let l3 = it.next().unwrap().as_ref();
            assert!(l3.starts_with("After:  ["));
            let after = parse_number_list(&l3[9..(l3.len() - 1)], ", ");
            assert_eq!(after.len(), 4);

            let l4 = it.next().unwrap().as_ref();
            assert!(l4.is_empty());

            let mut s = Sample::default();
            s.before.copy_from_slice(&before);
            s.ops.copy_from_slice(&ops);
            s.after.copy_from_slice(&after);
            v.push(s);
        }
        v
    }

    /// Return the instructions that could possibly have generated this output,
    /// from this input.
    pub fn possible_inst(&self) -> Vec<Inst> {
        let mut v = Vec::with_capacity(16);
        for inst in Inst::all().iter() {
            if inst.apply(&self.before, &self.ops) == self.after {
                v.push(*inst);
            }
        }
        v
    }
}

pub fn main() {
    use std::fs::File;
    use std::io::Read;
    let mut s = String::new();
    File::open("input/input16.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let ls: Vec<String> = s.lines().map(|l| l.to_string()).collect();
    let sams = Sample::parse_samples(&ls);
    let mut gt3 = 0;
    for s in sams {
        println!("{:?}", s);
        let poss = s.possible_inst();
        println!("possible: {:?}", poss);
        if poss.len() >= 3 {
            gt3 += 1;
        }
    }
    println!("Samples explainable by >=3 opcodes: {:?}", gt3);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_sample() {
        let ss = Sample::parse_samples(&[
            "Before: [3, 2, 1, 1]",
            "9 2 1 2",
            "After:  [3, 2, 2, 1]",
            "",
            "Before: [0, 2, 3, 2]",
            "5 0 2 1",
            "After:  [0, 0, 3, 2]",
            "",
        ]);
        assert_eq!(
            ss,
            vec![
                Sample {
                    before: [3, 2, 1, 1],
                    ops: [9, 2, 1, 2],
                    after: [3, 2, 2, 1],
                },
                Sample {
                    before: [0, 2, 3, 2],
                    ops: [5, 0, 2, 1],
                    after: [0, 0, 3, 2],
                }
            ]
        );
    }

    #[test]
    fn parse_number_list() {
        assert_eq!(
            super::parse_number_list("4, 3, 2, 1", ", "),
            vec![4, 3, 2, 1]
        );
    }

    #[test]
    fn simple_inst() {
        // muli (multiply immediate) stores into register C the result of
        // multiplying register A and value B.
        // r0 = r2 * 10
        assert_eq!(Muli.apply(&[1, 2, 3, 4], &[42, 2, 10, 0]), [30, 2, 3, 4]);

        // r0 = (r2 == r3)
        assert_eq!(
            Eqrr.apply(&[10, 20, 30, 40], &[42, 2, 3, 0]),
            [0, 20, 30, 40]
        );
        assert_eq!(
            Eqrr.apply(&[10, 20, 30, 30], &[42, 2, 3, 0]),
            [1, 20, 30, 30]
        );
    }

    #[test]
    fn possible() {
        let ss = Sample::parse_samples(&[
            "Before: [3, 2, 1, 1]",
            "9 2 1 2",
            "After:  [3, 2, 2, 1]",
            "",
        ]);
        assert_eq!(ss[0].possible_inst(), vec![Mulr, Addi, Seti]);
    }
}
