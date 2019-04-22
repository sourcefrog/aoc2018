#![allow(unused)]

/// https://adventofcode.com/2018/day/20
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

type Coord = i32;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Point {
    x: Coord,
    y: Coord,
}

/// Expand this regexp into a vector of every string that it can possibly match.
///
/// This is done by successively pulling off the first, top-level alternative group,
/// and enqueueing a new string to expand, for each of those alternatives.
/// This proceeds until there's nothing more to expand, at which point the string
/// is added to the result list.
fn expand(instr: &str) -> Vec<String> {
    // TODO: Maybe work in terms of indexes into the instruction list, rather than
    // copied strings?
    // TODO: Maybe store the queue as a constant part followed by a paren
    // part, to avoid repeatedly scanning the strings to find the first paren.

    // Queue of partially-completed expansions that we need to revisit. Each
    // might contain one or more alternates.
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(instr.to_string());
    let mut r = Vec::new();
    let mut ipaths = 0;
    while let Some(x) = queue.pop_front() {
        let (head, alts, tail) = parse_alternate(x);
        if alts.is_empty() {
            // If there were no alternates, there can't be a tail after them.
            debug_assert!(tail.is_empty());
            ipaths += 1;
            if ipaths % 100000 == 1 {
                println!("found {:?} paths, queue depth {}, recently {:?}", ipaths, queue.len(), head);
            }
            // r.push(head);
        } else {
            for a in alts {
                // Queue up each of these for later evaluation
                let mut b = head.clone();
                b.push_str(&a);
                b.push_str(&tail);
                queue.push_back(b);
            }
        }
    }
    r
}

/// Parse out the first alternate group from a string, returning the
/// head (before the alternate), a vec of alternatives, and then the
/// tail after it. Only the topmost alternative is parsed-out, so any of
/// the alts might themselves contain alternatives that later need to be expanded.
fn parse_alternate(s: String) -> (String, Vec<String>, String) {
    let sb = s.as_bytes();
    let mut i = 0;
    let mut head = String::new();
    // Pull off any head section before the parens
    loop {
        if i >= sb.len() {
            return (s, vec![], String::new()); // contains no paren
        } else if sb[i] == b'(' {
            head = s[..i].to_string();
            break;
        }
        i += 1;
    }
    // Skip over opening paren
    debug_assert_eq!(sb[i], b'(');
    i += 1;

    // Walk over alternates, accumulating each one into a vector, breaking
    // on top-level pipes or the closing bracket.
    let mut alts = Vec::new();
    let mut level = 0;
    let mut ai = i;
    loop {
        let c = sb[i];
        if level == 0 {
            match c {
                b'(' => {
                    level += 1;
                }
                b'|' => {
                    alts.push(s[ai..i].to_string());
                    ai = i + 1;
                }
                b')' => {
                    // conclude the last alternate, then stop
                    alts.push(s[ai..i].to_string());
                    i += 1;
                    break;
                }
                _other => (),
            }
        } else {
            // not at the top level, so just count parens for the sake of grouping, and skip
            // everything else.
            level += match c {
                b'(' => 1,
                b')' => -1,
                _other => 0,
            }
        }
        i += 1;
    }
    let tail = s[i..].to_string();
    (head, alts, tail)
}

fn walk(r: &str) {
    assert!(r.starts_with("^"));
    assert!(r.ends_with("$"));
    assert!(r.is_ascii());
    let r = &r[1..(r.len() - 1)];
    println!("r inner = {:?}", r);
    let cs: Vec<char> = r.chars().collect();
    // run_top(&cs);
}

/// Return the canned input with newline, ^ and $ removed.
fn load_input() -> String {
    let mut s = String::with_capacity(100_000);
    File::open("input/input20.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    s.shrink_to_fit();
    assert!(s.ends_with("$\n"));
    assert!(s.starts_with("^"));
    s[1..(s.len() - 2)].to_string()
}

pub fn main() {
    //walk("^WNE$");
    // walk("^WNE(EEE(SS|NN)|WWW|)$");
    // walk("^WNE(EEE(SS|NN)|WWW)$");
    // walk("^W((EE|SS)|(NN|WW))$");
    // walk("^WNE(EEE|WWW|)$");
}

#[cfg(test)]
mod test {
    use super::expand;
    use super::parse_alternate;

    fn check_parse(s: &str, head: &str, alts: Vec<&str>, tail: &str) {
        let alt_strings: Vec<String> = alts.into_iter().map(str::to_string).collect();
        assert_eq!(
            parse_alternate(s.to_string()),
            (head.to_string(), alt_strings, tail.to_string())
        );
    }

    #[test]
    fn test_parse_alternate() {
        check_parse("", "", vec![], "");
        check_parse("ABC", "ABC", vec![], "");
        check_parse("ABC()()", "ABC", vec![""], "()");
        check_parse("ABC()", "ABC", vec![""], "");
        check_parse("((N|S)|(E|W))", "", vec!["(N|S)", "(E|W)"], "");
        check_parse("NEWS((N|E|)EE|WW)", "NEWS", vec!["(N|E|)EE", "WW"], "");
        check_parse("NEWS(N|E|)EE", "NEWS", vec!["N", "E", ""], "EE");
    }

    #[test]
    fn test_expand() {
        assert_eq!(expand(""), vec![""]);
        assert_eq!(expand("N"), vec!["N"]);
        assert_eq!(expand("NEW"), vec!["NEW"]);
        assert_eq!(expand("(N|E)"), vec!["N", "E"]);
        assert_eq!(expand("((N|S)|(E|W))"), vec!["N", "S", "E", "W"]);
        assert_eq!(
            expand("NEWS((N|E|)EE|WW)"),
            vec!["NEWSWW", "NEWSNEE", "NEWSEEE", "NEWSEE"]
        );
    }

    #[test]
    fn test_load_input() {
        super::load_input();
    }

    #[test]
    fn test_expand_input() {
        let inst = super::load_input();
        println!("{:?} bytes of input", inst.len());
        let exp = expand(&inst);
        println!("{:?} paths in output; first is {:?}", exp.len(), exp[0]);
    }
}
