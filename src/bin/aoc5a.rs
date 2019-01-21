/// https://adventofcode.com/2018/day/5
use std::io;
use std::io::prelude::*;
use std::iter::Iterator;

pub fn main() {
    let lines = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    assert_eq!(lines.len(), 1);
    let c = collapse(&lines[0]);
    // println!("Collapsed to: {:?}", c);
    println!("len: {}", c.len());
}

fn matches(c0: char, c1: char) -> bool {
    c0.to_ascii_lowercase() == c1.to_ascii_lowercase()
        && c0.is_ascii_uppercase() != c1.is_ascii_uppercase()
}

/// Repeatedly remove matched letters from s; return the remnant.
fn collapse(s: &str) -> String {
    // Keep a stack of as-yet unmatched characters. Consume from the input
    // one character at a time; if it matches the top character from the stack
    // then pop that off (and also discard the new one); otherwise push.
    let mut st: Vec<char> = Vec::new();
    for c1 in s.chars() {
        if let Some(c0) = st.pop() {
            if matches(c0, c1) {
                // println!("{:?} ** {:?}", c0, c1);
                // discard both
            } else {
                st.push(c0);
                st.push(c1);
            }
        } else {
            // Stack's already empty; must push
            st.push(c1);
        }
    }
    st.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::collapse;

    #[test]
    fn simple_collapse() {
        assert_eq!(collapse("ab"), "ab");
        assert_eq!(collapse("aA"), "");
        assert_eq!(collapse("BaAb"), "");
        assert_eq!(collapse("abAB"), "abAB");
        assert_eq!(collapse("aabAAB"), "aabAAB");
    }

    #[test]
    fn repeated_collaps() {
        assert_eq!(collapse("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
    }
}
