use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

/// Number of registers.
const NREG: usize = 6;

type Reg = usize;

/// An instruction
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Opcode {
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
use self::Opcode::*;

impl Opcode {
    pub fn all() -> &'static [Opcode] {
        &[
            Muli, Mulr, Addr, Addi, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir,
            Eqri, Eqrr,
        ]
    }
}

impl FromStr for Opcode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "addr" => Addr,
            "addi" => Addi,
            "mulr" => Mulr,
            "muli" => Muli,
            "banr" => Banr,
            "bani" => Bani,
            "borr" => Borr,
            "bori" => Bori,
            "setr" => Setr,
            "seti" => Seti,
            "gtir" => Gtir,
            "gtri" => Gtri,
            "gtrr" => Gtrr,
            "eqir" => Eqir,
            "eqri" => Eqri,
            "eqrr" => Eqrr,
            _ => return Err(()),
        })
    }
}

/// An instruction with opcode and arguments.
#[derive(Debug, PartialEq, Eq)]
struct Inst {
    opcode: Opcode,
    a: Reg,
    b: Reg,
    c: Reg,
}

impl Inst {
    /// Apply an instruction to some registers; return new registers.
    pub fn apply(&self, r: &[Reg; NREG]) -> [Reg; NREG] {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        let mut after = *r;
        // Happily in Rust if you cast a bool to an int, it goes to 1 and 0.
        after[c] = match self.opcode {
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
}

impl FromStr for Inst {
    type Err = ();

    fn from_str(s: &str) -> Result<Inst, ()> {
        let s = s.trim();
        let opcode = Opcode::from_str(&s[0..4])?;
        let mut args = s[5..].split(' ');
        let a = args.next().unwrap().parse().unwrap();
        let b = args.next().unwrap().parse().unwrap();
        let c = args.next().unwrap().parse().unwrap();
        Ok(Inst { opcode, a, b, c })
    }
}

pub struct Program {
    /// Index of the register bound to the IP.
    ip_reg: Reg,

    /// Current instruction pointer.
    ip: usize,

    /// All the registers.
    reg: [Reg; NREG],

    /// Instructions
    code: Vec<Inst>,
}

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Program, Self::Err> {
        let mut lines = s.lines();
        let mut code = Vec::new();

        let ip_l = lines.next().unwrap();
        assert!(ip_l.starts_with("#ip "));
        let ip_reg = ip_l[4..].parse().unwrap();

        for l in lines {
            code.push(l.parse().unwrap());
        }

        Ok(Program {
            ip_reg,
            ip: 0,
            reg: [0; NREG],
            code,
        })
    }
}

impl Program {
    /// Run the program as long as the IP is valid; then return the contents of
    /// register 0.
    pub fn eval(&mut self) -> Reg {
        while self.ip < self.code.len() {
            self.reg[self.ip_reg] = self.ip;
            let new_reg = self.code[self.ip].apply(&self.reg);
            // println!(
            //     "ip={} {:?} inst={:?} {:?}",
            //     self.ip, self.reg, self.code[self.ip], new_reg
            // );

            self.reg = new_reg;
            self.ip = self.reg[self.ip_reg] + 1;
        }
        self.reg[0]
    }
}

pub fn solve() -> Reg {
    let mut s = String::new();
    File::open("input/input19.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    Program::from_str(&s).unwrap().eval()
}

pub fn main() {
    println!("final reg 0: {}", solve());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let ptext = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5
";
        let mut p = Program::from_str(&ptext).unwrap();
        assert_eq!(p.code.len(), 7);
        assert_eq!(
            p.code[4],
            Inst {
                opcode: Setr,
                a: 1,
                b: 0,
                c: 0
            }
        );
        assert_eq!(p.eval(), 6);
    }

    #[test]
    fn parse_opcode() {
        assert_eq!("seti".parse(), Ok(Seti));
    }

    #[test]
    fn parse_inst() {
        assert_eq!(
            "addi 4 13 4".parse(),
            Ok(Inst {
                opcode: Addi,
                a: 4,
                b: 13,
                c: 4
            })
        )
    }

    #[test]
    fn overall_result() {
        assert_eq!(solve(), 1302);
    }
}
