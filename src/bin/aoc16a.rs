#![allow(dead_code)]

/// Infer opcode numbers from examples.
use regex;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Sample {
    /// Register values before the operation.
    before: [u8; 4],
    /// Encoded instruction, consisting of an opcode followed by three arguments.
    ops: [u8; 4],
    /// Regesters afterwards.
    after: [u8; 4],
}

fn parse_number_list(s: &str, sep: &str) -> Vec<u8> {
    let mut v = Vec::with_capacity(4);
    for a in s.split(sep) {
        v.push(a.parse().unwrap());
    }
    v
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
}
