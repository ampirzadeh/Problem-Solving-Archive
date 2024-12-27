use advent_of_code_2024::{Point, Solution};
use itertools::Itertools;

const TRAIL_HEAD: i128 = 0;
const FINAL_TARGET: i128 = 9;

pub struct Day10 {
    pub input: String,
}

impl Day10 {
    fn get_matrix(&self) -> Vec<Vec<i128>> {
        self.input
            .lines()
            .map(|x| {
                x.chars()
                    .map(|ch| ch.to_digit(10).unwrap() as i128)
                    .collect_vec()
            })
            .collect_vec()
    }

    fn get_point(&self, point: Point) -> Option<i128> {
        let matrix = self.get_matrix();
        matrix
            .get(point.x as usize)
            .and_then(|f| f.get(point.y as usize))
            .copied()
    }

    fn check_adjacent(&self, point: Point, target: i128) -> Vec<Point> {
        let mut reachable_nines = vec![];
        let next_target = target + 1;

        let to_checks = vec![
            point + Point::new(0, 1), // right
            point + Point::new(1, 0), // down
            point - Point::new(0, 1), // left
            point - Point::new(1, 0), // top
        ];

        for to_check in to_checks {
            if let Some(val) = self.get_point(to_check) {
                if val == target {
                    if target == FINAL_TARGET {
                        reachable_nines.push(to_check);
                    } else {
                        reachable_nines.extend(self.check_adjacent(to_check, next_target));
                    }
                }
            }
        }

        reachable_nines
    }
}

impl Solution for Day10 {
    fn part1(&self) -> String {
        let mut nines_reached: Vec<Point> = vec![];

        for (row, roww) in self.get_matrix().iter().enumerate() {
            for (col, coll) in roww.iter().enumerate() {
                if coll.to_owned() == TRAIL_HEAD {
                    nines_reached.extend(
                        self.check_adjacent(Point::new(row as i128, col as i128), TRAIL_HEAD + 1)
                            .iter()
                            .unique()
                            .collect_vec(),
                    );
                }
            }
        }

        nines_reached.len().to_string()
    }

    fn part2(&self) -> String {
        let mut nines_reached: Vec<Point> = vec![];

        for (row, roww) in self.get_matrix().iter().enumerate() {
            for (col, coll) in roww.iter().enumerate() {
                if coll.to_owned() == TRAIL_HEAD {
                    nines_reached.extend(
                        self.check_adjacent(Point::new(row as i128, col as i128), TRAIL_HEAD + 1),
                    );
                }
            }
        }

        nines_reached.len().to_string()
    }
}
