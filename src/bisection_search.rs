use std::fmt::Debug;
use std::ops::{Add, Div};

/// Find the smallest value of `v` such that `f` returns true.
///
/// `f` must be such that it's true for all larger values.
///
/// ```
/// use aoc2018::bisection_search;
///
/// assert_eq!(bisection_search(0u32, 200u32, |_| true), Some(0));
///
/// assert_eq!(bisection_search(0u32, 200u32, |_| false), None);
///
/// assert_eq!(bisection_search(0u32, 200u32, |i| i>17), Some(18));
/// ```
pub fn bisection_search<V, F>(mut v_min: V, mut v_max: V, f: F) -> Option<V>
where
    F: Fn(V) -> bool,
    V: Ord + Add<V, Output = V> + Div<Output = V> + Copy + Debug + From<u16>,
{
    if !f(v_max) {
        // not found even at the highest values
        return None;
    }
    // Otherwise, there must be some value in v_min..=v_max that matches.

    loop {
        if v_min == v_max {
            // If it's anywhere it must be here.
            // println!("converged to match at {:?}", v_min);
            return Some(v_min);
        }
        let mid = (v_min + v_max) / From::from(2);
        if f(mid) {
            // True in the middle so let's keep looking lower.
            v_max = mid;
        // println!("step down to {:?}..={:?}", v_min, v_max);
        } else {
            v_min = mid + From::from(1);
            // println!("step up to {:?}..={:?}", v_min, v_max);
        }
    }
}
