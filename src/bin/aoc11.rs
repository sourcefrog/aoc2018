/// https://adventofcode.com/2018/day/11
use std::cmp::min;

use aoc2018::Matrix;
use aoc2018::{point, Point};

// Performance can probably be improved by remembering the sum of some
// (aligned? even sized?) blocks and using them when computing the sum of
// larger enclosing blocks. But the brute force approach works in a basically
// acceptable amount of time; about 50s.
//
// Or: roll lines of squares into and out of the currently computed square,
// as it moves across the page, rather than summing up every square as we go.
// And in fact, to move down a line, we need only roll one set of squares in
// and one set out.

const SIZE: usize = 300;

fn solve_a() -> ((usize, usize), i32) {
    Map::new(7672).hottest(3)
}

fn solve_b() -> ((usize, usize), usize, i32) {
    Map::new(7672).any_hottest()
}

pub fn main() {
    println!("best of size 3: {:?}", solve_a());
    println!("best of any size: {:?}", solve_b());
}

struct Map {
    /// Power levels indexed by `point(x, y)`.
    /// In the problem description indices are 1-based but for simplicity
    /// these are 1-based, and we convert on output.
    p: Matrix<i32>,
}

impl Map {
    pub fn new(grid: i32) -> Map {
        let mut p = Matrix::new(SIZE + 1, SIZE + 1, i32::min_value());
        for x in 0..SIZE {
            for y in 0..SIZE {
                // Find the fuel cell's rack ID, which is its X coordinate
                // plus 10.
                let rack_id = (x + 1) as i32 + 10;
                // Begin with a power level of the rack ID times the Y coordinate.
                let mut pwr: i32 = rack_id * (y + 1) as i32;
                // Increase the power level by the value of the grid serial
                // number (your puzzle input).
                pwr += grid;
                // Set the power level to itself multiplied by the rack ID.
                pwr *= rack_id;
                // Keep only the hundreds digit of the power level (so 12345 becomes 3;
                // numbers with no hundreds digit become 0).
                pwr = (pwr / 100) % 10;
                // Subtract 5 from the power level.
                pwr -= 5;
                p[point(x, y)] = pwr;
            }
        }
        Map { p }
    }

    pub fn get(&self, c: (usize, usize)) -> i32 {
        self.p[point(c.0, c.1)]
    }

    pub fn squaresum(&self, c: (usize, usize), sqsz: usize) -> i32 {
        let mut s: i32 = 0;
        for x in c.0..(c.0 + sqsz) {
            for y in c.1..(c.1 + sqsz) {
                s += self.p[point(x, y)];
            }
        }
        s
    }

    pub fn hottest(&self, sqsz: usize) -> ((usize, usize), i32) {
        let mut best_power: i32 = i32::min_value();
        let mut best_point: (usize, usize) = (0, 0);

        for x in 0..(SIZE - sqsz) {
            for y in 0..(SIZE - sqsz) {
                let p = (x, y);
                let pwr = self.squaresum(p, sqsz);
                if pwr > best_power {
                    best_power = pwr;
                    best_point = p;
                }
            }
        }
        ((best_point.0 + 1, best_point.1 + 1), best_power)
    }

    /// Return the sum of power within a square of size `sqsz` at `p`, given
    /// the sum of the power within a square of `sqsz-1`. This can be done
    /// by just adding the values along the new expanded border.
    ///
    /// This is safe to call even with sqsz==1 (and oldpow==0).
    fn grow_square(&self, sqsz: usize, p: Point, oldpow: i32) -> i32 {
        // Suppose we've already calculated the 3x3 square starting at (1,2)
        // and we want to extend that to 4x4.
        //
        // ..........
        // .xxx*.....
        // .xxx*.....
        // .xxx*.....
        // .***@.....
        //
        // We need to add the column of 3 (sqsz-1) cells at p.x+sqsz-1
        // running from p.y down to p.y+oldsz-1. Similarly for the row
        // running across. And then finally the single corder square marked
        // @, at p.x+oldsz, p.y+oldsz.
        let mut newpow = oldpow;
        debug_assert!(sqsz >= 1);
        let oldsz = sqsz - 1;
        for i in 0..oldsz {
            newpow += self.p[point(p.x + oldsz, p.y + i)] + self.p[point(p.x + i, p.y + oldsz)];
        }
        // And count the corner, but only once.
        newpow += self.get((p.x + oldsz, p.y + oldsz));
        newpow
    }

    /// Find the square within the map that has the largest total power.
    ///
    /// Returns the (x,y) coords of the top of that square, its size, and the
    /// total power.
    #[allow(dead_code)]
    pub fn hottest_square(&self) -> ((usize, usize), usize, i32) {
        // General approach here is to work up through squares of increasing
        // sizes, starting from 1.
        //
        // As we go along, we simply remember the origin, size, and total power
        // of the most powerful cell we've seen.
        //
        // As we go along we remember the sum of power of strips of size
        // S running vertically down from every possible cell, and also
        // horizontally across from every possible cell. (Not from those
        // within S of the boundary.) We also remember the sum of power
        // for squares of size S in every possible position.
        //
        // To start with at S=1 these are all trivially the value of each
        // cell itself.
        //
        // To proceed to S+1, we first extend each of the squares
        // by adding in the vertical strip for the next column, and the
        // horizontal strip for the next row, and the single cell in the
        // corner between them. Then, we extend the strips by adding in
        // one more square in each direction.

        unimplemented!();
    }

    /// Find the square within the map that has the largest total power.
    ///
    /// Returns the (x,y) coords of the top of that square, its size, and the
    /// totaly power.
    pub fn any_hottest(&self) -> ((usize, usize), usize, i32) {
        let mut best_point = (0, 0);
        let mut best_power = i32::min_value();
        let mut best_size = 1;
        for x in 0..SIZE {
            for y in 0..SIZE {
                let p = point(x, y);
                // Gradually add up the power of increasing-sized squares at p.
                let mut pwr = 0;
                for sqsz in 1..min(SIZE - x, SIZE - y) {
                    pwr = self.grow_square(sqsz, p, pwr);
                    if pwr > best_power {
                        best_size = sqsz;
                        best_power = pwr;
                        best_point = (p.x, p.y);
                    }
                }
            }
        }
        ((best_point.0 + 1, best_point.1 + 1), best_size, best_power)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(Map::new(57).get((122 - 1, 79 - 1)), -5);

        // Fuel cell at 217,196, grid serial number 39: power level  0.
        assert_eq!(Map::new(39).get((217 - 1, 196 - 1)), 0);

        // Fuel cell at 101,153, grid serial number 71: power level  4.
        assert_eq!(Map::new(71).get((101 - 1, 153 - 1)), 4);
    }

    #[test]
    fn squaresum_examples() {
        let m = Map::new(18);
        assert_eq!(m.squaresum((33 - 1, 45 - 1), 3), 29);
        assert_eq!(m.hottest(3), ((33, 45), 29));

        let m = Map::new(42);
        assert_eq!(m.squaresum((21 - 1, 61 - 1), 3), 30);
        assert_eq!(m.hottest(3), ((21, 61), 30));
    }

    #[test]
    fn variable_size() {
        // For grid serial number 18, the largest total square (with a total power of 113) is 16x16 and has a top-left corner of 90,269, so its identifier is 90,269,16.
        assert_eq!(Map::new(18).any_hottest(), ((90, 269), 16, 113));
    }

    #[test]
    fn variable_size_2() {
        // For grid serial number 42, the largest total square (with a total power of 119) is 12x12 and has a top-left corner of 232,251, so its identifier is 232,251,12.
        assert_eq!(Map::new(42).any_hottest(), ((232, 251), 12, 119));
    }

    #[test]
    fn part_a_solution() {
        assert_eq!(super::solve_a(), ((22, 18), 29));
    }

    #[test]
    fn part_b_solution() {
        assert_eq!(super::solve_b(), ((234, 197), 14, 98));
    }
}
