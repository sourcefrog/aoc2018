// Recipes are only ever appended to the board.

pub fn main() {
    println!("{}", Board::new().scores_after(890691));
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Board {
    r: Vec<u8>,

    /// Index of the current recipe of Elf 1.
    e1: usize,
    e2: usize,
}

impl Board {
    pub fn new() -> Board {
        Board {
            r: vec![3, 7],
            e1: 0,
            e2: 1,
        }
    }

    pub fn step(&mut self) {
        // Sum the current recipes
        let s = self.r[self.e1] + self.r[self.e2];
        // Create new recipes
        if s >= 10 {
            assert_eq!(s / 10, 1);
            self.r.push(1);
            self.r.push(s % 10);
        } else {
            self.r.push(s);
        }
        // Advance
        self.e1 = (self.e1 + 1 + self.r[self.e1] as usize) % self.r.len();
        self.e2 = (self.e2 + 1 + self.r[self.e2] as usize) % self.r.len();
    }

    /// Return, as a string, the digits after the first l recipes.
    pub fn scores_after(&mut self, l: usize) -> String {
        const N: usize = 10;
        while self.r.len() < (l + N) {
            self.step();
        }
        let v = &self.r[(l)..(l + N)];
        assert_eq!(v.len(), N);
        let vs: Vec<String> = v.iter().map(|d| format!("{}", d)).collect();
        vs.join("")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let mut b = Board::new();
        assert_eq!(b.r, vec![3, 7]);

        b.step();
        assert_eq!(b.r, vec![3, 7, 1, 0]);
        assert_eq!(b.e1, 0);
        assert_eq!(b.e2, 1);

        b.step();
        assert_eq!(b.r, vec![3, 7, 1, 0, 1, 0]);
        assert_eq!(b.e1, 4);
        assert_eq!(b.e2, 3);

        for _ in 3..=15 {
            b.step();
        }
        assert_eq!(
            b.r,
            vec![3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6, 7, 7, 9, 2]
        );
    }

    #[test]
    fn examples() {
        assert_eq!(Board::new().scores_after(9), "5158916779");
        assert_eq!(Board::new().scores_after(5), "0124515891");
        assert_eq!(Board::new().scores_after(18), "9251071085");
        assert_eq!(Board::new().scores_after(2018), "5941429882");
    }
}
