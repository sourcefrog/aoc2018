//! https://adventofcode.com/2018/day/2

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

/// True if the two strings differ by exactly one character at the same position
fn onediff(a: &str, b: &str) -> bool {
    a.chars().zip(b.chars()).filter(|(aa, bb)| aa != bb).count() == 1
}

/// Say whether s has, respectively
/// - any letters that occur exactly twice each
/// - any letters that occur exactly three times
fn pat(s: &str) -> (bool, bool) {
    let mut counts = HashMap::<char, u32>::new();
    for c in s.chars() {
        counts.insert(c, counts.get(&c).unwrap_or(&0) + 1);
    }
    // Were there any characters present exactly 2 times, or exactly 3?
    (
        counts.values().any(|x| *x == 2),
        counts.values().any(|x| *x == 3),
    )
}

fn checksum<S: AsRef<str>>(ss: &[S]) -> i64 {
    let pats: Vec<(bool, bool)> = ss.iter().map(|sp| pat(sp.as_ref())).collect();
    pats.iter().filter(|(two, _)| *two).count() as i64
        * pats.iter().filter(|(_, three)| *three).count() as i64
}

/// Common characters between two strings
fn common(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter_map(|(aa, bb)| if aa == bb { Some(aa) } else { None })
        .collect()
}

fn find_close<S: AsRef<str>>(ls: Vec<S>) -> Option<String> {
    for i in ls.iter() {
        for j in ls.iter() {
            let i = i.as_ref();
            let j = j.as_ref();
            if onediff(i, j) {
                return Some(common(i, j));
            }
        }
    }
    None
}

pub fn main() {
    let ls: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    println!("checksum: {}", checksum(ls.as_slice()));
    println!("{}", find_close(ls).unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pat() {
        assert_eq!(super::pat("hello"), (true, false));
        assert_eq!(super::pat("abcdef"), (false, false));
        assert_eq!(super::pat("bababc"), (true, true));
        assert_eq!(super::pat("abbcde"), (true, false));
        assert_eq!(super::pat("abcccd"), (false, true));
        assert_eq!(super::pat("aabcdd"), (true, false));
        assert_eq!(super::pat("abcdee"), (true, false));
        assert_eq!(super::pat("ababab"), (false, true));
    }

    #[test]
    fn test_checksum() {
        assert_eq!(
            super::checksum(&[
                "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"
            ]),
            12
        );
    }
    #[test]
    fn test_onediff() {
        use super::onediff;

        assert_eq!(onediff("abcde", "axcye"), false);
        assert_eq!(onediff("abcde", "wvxyz"), false);
        assert_eq!(onediff("fghij", "axcye"), false);
        assert_eq!(onediff("fghij", "fguij"), true);
    }

    #[test]
    fn find_close() {
        use super::find_close;
        assert_eq!(
            find_close(vec![
                "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
            ]),
            Some("fgij".to_string())
        );
    }
}
