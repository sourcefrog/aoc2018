/// https://adventofcode.com/2018/day/11

// Performance can probably be improved by remembering the sum of some
// (aligned? even sized?) blocks and using them when computing the sum of
// larger enclosing blocks. But the brute force approach works in a basically
// acceptable amount of time; about 50s.

const SIZE: usize = 301;

pub fn main() {
    println!("best of size 3: {:?}", Map::new(7672).hottest(3));
    println!("best of any size: {:?}", Map::new(7672).any_hottest());
}

struct Map {
    /// Power levels indexed by [x][y], with indexes 1-based.
    /// (So, 0 coordinates are wasted.)
    p: Vec<Vec<i32>>,

    #[allow(unused)]
    grid: i32,
}

impl Map {
    pub fn new(grid: i32) -> Map {
        let mut p: Vec<Vec<i32>> = Vec::with_capacity(SIZE);
        for x in 0..SIZE {
            let mut pp: Vec<i32> = Vec::with_capacity(SIZE);
            for y in 0..SIZE {
                // Find the fuel cell's rack ID, which is its X coordinate plus 10.
                let rack_id = x as i32 + 10;
                // Begin with a power level of the rack ID times the Y coordinate.
                let mut pwr: i32 = rack_id * y as i32;
                // Increase the power level by the value of the grid serial number (your puzzle input).
                pwr += grid;
                // Set the power level to itself multiplied by the rack ID.
                pwr *= rack_id;
                // Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
                pwr = (pwr / 100) % 10;
                // Subtract 5 from the power level.
                pwr -= 5;
                pp.push(pwr);
            }
            p.push(pp);
        }
        Map { p, grid }
    }

    pub fn get(&self, c: (usize, usize)) -> i32 {
        let (x, y) = c;
        self.p[x][y]
    }

    pub fn squaresum(&self, c: (usize, usize), sqsz: usize) -> i32 {
        let mut s: i32 = 0;
        for x in c.0..(c.0+sqsz) {
            for y in c.1..(c.1+sqsz) {
                s += self.get((x, y));
            }
        }
        s
    }

    pub fn hottest(&self, sqsz: usize) -> ((usize, usize), i32) {
        let mut best_power: i32 = i32::min_value();
        let mut best_point: (usize, usize) = (0, 0);

        for x in 1..=(SIZE-sqsz) {
            for y in 1..=(SIZE-sqsz) {
                let p = (x, y);
                let pwr = self.squaresum(p, sqsz);
                if pwr > best_power {
                    best_power = pwr;
                    best_point = p;
                }
            }
        }
        (best_point, best_power)
    }

    pub fn any_hottest(&self) -> ((usize, usize), usize, i32) {
        let mut best_point = (0, 0);
        let mut best_power = i32::min_value();
        let mut best_size = 1;
        for sqsz in 1..=SIZE {
            let (p, pwr) = self.hottest(sqsz);
            if pwr > best_power {
                best_size = sqsz;
                best_power = pwr;
                best_point = p;
            }
        }
        (best_point, best_size, best_power)
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(Map::new(57).get((122, 79)), -5);

        // Fuel cell at 217,196, grid serial number 39: power level  0.
        assert_eq!(Map::new(39).get((217, 196)), 0);

        // Fuel cell at 101,153, grid serial number 71: power level  4.
        assert_eq!(Map::new(71).get((101, 153)), 4);
    }

    #[test]
    fn squaresum_examples() {
        let m = Map::new(18);
        assert_eq!(m.squaresum((33, 45), 3), 29);
        assert_eq!(m.hottest(3), ((33, 45), 29));

        let m = Map::new(42);
        assert_eq!(m.squaresum((21, 61), 3), 30);
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
}