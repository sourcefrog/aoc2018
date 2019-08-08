/// https://adventofcode.com/2018/day/8
use std::io;
use std::io::prelude::*;

pub fn main() {
    let nums = parse_ints(&io::stdin().lock().lines().next().unwrap().unwrap());
    println!("root node value = {:?}", node_value(&mut nums.into_iter()))
}

/// Read a node and any child nodes out of the iterator, and return the sum of
/// their metadata.
fn node_value<I: Iterator<Item = usize>>(l: &mut I) -> usize {
    let nkids = l.next().unwrap();
    let nmeta = l.next().unwrap();
    let mut child_values: Vec<usize> = Vec::with_capacity(nkids);
    for _i in 0..nkids {
        child_values.push(node_value(l));
    }
    if nkids == 0 {
        let mut tot = 0;
        for _i in 0..nmeta {
            tot += l.next().unwrap();
        }
        tot
    } else {
        let mut tot = 0;
        for _i in 0..nmeta {
            let m = l.next().unwrap();
            if m > 0 && m <= child_values.len() {
                tot += child_values[m - 1];
            }
        }
        tot
    }
}

pub fn parse_ints(s: &str) -> Vec<usize> {
    s.split(' ')
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let nums = parse_ints("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(node_value(&mut nums.into_iter()), 66);
    }
}
