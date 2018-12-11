/// https://adventofcode.com/2018/day/8
// use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::io::prelude::*;

pub fn main() {
    let nums = parse_ints(&io::stdin().lock().lines().next().unwrap().unwrap());
    println!("{:?}", nums);
    println!(
        "sum of metadata = {:?}",
        sum_metadata(&mut nums.into_iter())
    )
}

/// Read a node and any child nodes out of the iterator, and return the sum of
/// their metadata.
pub fn sum_metadata(l: &mut Iterator<Item = u32>) -> u32 {
    let nkids = l.next().unwrap();
    let nmeta = l.next().unwrap();
    let mut tot = 0;
    for _i in 0..nkids {
        tot += sum_metadata(l);
    }
    for _i in 0..nmeta {
        tot += l.next().unwrap();
    }
    tot
}

pub fn parse_ints(s: &str) -> Vec<u32> {
    s.split(' ')
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let nums = parse_ints("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(sum_metadata(&mut nums.into_iter()), 138);
    }
}
