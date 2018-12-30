#![allow(dead_code)]

//! Infer opcode numbers from examples.

use std::collections::BTreeMap;
use std::ops::Range;

use regex;

type Reg = usize;
const OPS: Range<Reg> = 0..16;

/// An instruction
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Inst {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}
use self::Inst::*;

impl Inst {
    pub fn apply(self, r: &[Reg; 4], args: &[Reg]) -> [Reg; 4] {
        debug_assert_eq!(args.len(), 4);
        let (a, b, c) = (args[1], args[2], args[3]);
        let mut after = r.clone();
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
            assert!(l1.ends_with("]"));
            let before = parse_number_list(&l1[9..(l1.len() - 1)], ", ");
            assert_eq!(before.len(), 4);

            let l2 = it.next().unwrap().as_ref();
            let ops = parse_number_list(l2, " ");
            assert_eq!(ops.len(), 4);

            let l3 = it.next().unwrap().as_ref();
            assert!(l3.starts_with("After:  ["));
            let after = parse_number_list(&l3[9..(l3.len() - 1)], ", ");
            assert_eq!(after.len(), 4);

            let l4 = it.next();
            assert!(l4.is_none() || l4.unwrap().as_ref().is_empty());

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

/// Given some samples, work out which opcodes could possibly have what effect.
#[derive(Debug, Default)]
pub struct Infer {
    /// Potential Opcode->Instructions mapping that have been observed.
    possible: Vec<Vec<Inst>>,

    /// Opcode-instruction mapping that's been unambiguously determined.
    certain: Vec<Option<Inst>>,
}

impl Infer {
    fn mark_possible(&mut self, inst: Inst, opcode: Reg) {
        let ops = &mut self.possible[opcode];
        if !ops.contains(&inst) {
            ops.push(inst);
        }
    }

    pub fn new(sams: &[Sample]) -> Infer {
        let mut possible = Vec::with_capacity(16);
        let mut certain = Vec::with_capacity(16);
        for _opcode in OPS {
            possible.push(Vec::new());
            certain.push(None);
        }
        let mut inf = Infer { possible, certain };
        for s in sams {
            // println!("{:?}", s);
            let pi = s.possible_inst();
            let opcode = s.ops[0];
            // println!("possible: {:?} => {:?}", opcode, pi);
            for inst in pi {
                inf.mark_possible(inst, opcode);
            }
        }
        inf
    }

    /// Eliminate unambiguous encodings until we know all the opcodes.
    ///
    /// This isn't guaranteed to terminate if we reach a point where there's
    /// no single step without backtracking, but let's try it.
    pub fn reduce(&mut self) -> Decode {
        let mut solved_inst = BTreeMap::<Inst, Reg>::new();
        while solved_inst.len() < 16 {
            for (opcode, insts) in self.possible.iter().enumerate() {
                // Look for opcodes that could decode to only one instruction
                // whose value isn't already known.
                let unresolved: Vec<Inst> = insts
                    .iter()
                    .filter(|inst| !solved_inst.contains_key(inst))
                    .map(|ri| *ri)
                    .collect();
                if unresolved.len() == 1 {
                    let inst = unresolved[0];
                    println!("Found certainly {} === {:?}", opcode, inst);
                    debug_assert!(self.certain[opcode].is_none());
                    self.certain[opcode] = Some(inst);
                    solved_inst.insert(inst, opcode);
                }
            }
        }
        Decode {
            op_ins: self.certain.iter().cloned().map(Option::unwrap).collect(),
        }
    }
}

#[derive(Debug)]
pub struct Decode {
    // Opcode -> Instruction mapping
    op_ins: Vec<Inst>,
}

pub struct Program {
    asm: Vec<Vec<Reg>>,
}

impl Program {
    pub fn parse(s: &str) -> Program {
        let mut asm = Vec::new();
        for l in s.lines() {
            if l.is_empty() {
                continue;
            };
            asm.push(parse_number_list(l, " "));
        }
        Program { asm }
    }

    pub fn eval(&self, decode: &Decode) {
        let mut r = [0; 4];
        for p in self.asm.iter() {
            let inst = decode.op_ins[p[0]];
            println!("{:?} {:?}", inst, &p[1..]);
            r = inst.apply(&r, &p);
            println!(" => {:?}", r);
        }
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
    // Split examples from program
    let mut splits = s.split("\n\n\n");
    let samples = splits.next().unwrap();
    let ls: Vec<String> = samples.lines().map(|l| l.to_string()).collect();
    let sams = Sample::parse_samples(&ls);
    let mut infer = Infer::new(&sams);
    let decode = infer.reduce();
    let prog = Program::parse(splits.next().unwrap());
    prog.eval(&decode);
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
