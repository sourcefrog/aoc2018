use aoc2018::{point, Point, Matrix};

type Erosion = usize;

/// Make a matrix from (0,0) through `target`, inclusive, with values
/// being the erosion level of each cell.
fn make_geo_map(depth: usize, target: Point) -> Matrix<Erosion> {
    let mut map = Matrix::new(target.x + 1, target.y + 1,
        0 as Erosion);
    for x in 0..=target.x {
        for y in 0..=target.y {
            let v = if point(x, y) == target {
                0
            } else if y == 0 {
                // This also handles the (0,0) case.
                x * 16807
            } else if x == 0 {
                y * 48271
            } else {
                let p1 = point(x-1, y);
                let p2 = point(x, y-1);
                let v1 = map[p1];
                let v2 = map[p2];
                match v1.checked_mul(v2) {
                    Some(v) => v,
                    None => panic!("failed to multiply at ({}, {}): \
                        p1={:?}, p2={:?}, v1={:?}, v2={:?}", x, y, p1, p2, v1, v2),
                }
            };
            let v = (v + depth) % 20183;
            let p = point(x, y);
            // dbg!(p, v);
            map[p] = v;
        }
    }
    map
}

pub fn calc_risk(map: &Matrix<Erosion>) -> usize {
    // The modulus calculation of region type exactly corresponds to the
    // risk of each region: 0=rocky, 1=wet, 2=narrow.
    map.values()
        .map(|e| { e % 3 })
        .sum()
}

pub fn solve() -> usize {
    calc_risk(&make_geo_map(5616, point(10, 785)))
}

pub fn main() {
    println!("Result: {}", solve());;
}

#[cfg(test)]
mod test {
    use aoc2018::point;
    use super::{calc_risk, make_geo_map};

    #[test]
    fn build_map() {
        let map = make_geo_map(510, point(10, 10));
        assert_eq!(map[point(0, 0)], 510);
        assert_eq!(map[point(1, 0)], 17317);
        assert_eq!(map[point(0, 1)], 8415);
        assert_eq!(map[point(1, 1)], 1805);
        assert_eq!(map[point(10, 10)], 510);

        assert_eq!(calc_risk(&map), 114);
    }

    #[test]
    fn expected_solution() {
        assert_eq!(super::solve(), 8681);
    }
}
