use std::collections::HashSet;

use advent_of_code_2024::{Point, Solution};
use itertools::Itertools;

pub struct Day6 {
    pub input: String,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Down,
    Right,
}

impl Day6 {
    fn get_matrix(&self) -> Vec<Vec<char>> {
        self.input.lines().map(|x| x.chars().collect()).collect()
    }

    fn get_point(&self, matrix: &Vec<Vec<char>>, point: Point) -> Option<char> {
        matrix
            .get(point.x as usize)
            .and_then(|f| f.get(point.y as usize))
            .copied()
    }

    fn get_starting_info(&self) -> (Point, Direction) {
        let matrix = self.get_matrix();
        let height = matrix.len();
        let width = matrix[0].len();

        let mut starting_pos: Option<Point> = None;
        let mut starting_dir: Option<Direction> = None;

        for i in 0..height {
            for j in 0..width {
                let chr = matrix[i][j];

                if ['<', '>', 'v', '^'].contains(&chr) {
                    starting_pos = Some(Point::new(i as i128, j as i128));

                    starting_dir = match chr {
                        '>' => Some(Direction::Right),
                        'v' => Some(Direction::Down),
                        '<' => Some(Direction::Left),
                        '^' => Some(Direction::Up),
                        _ => None,
                    }
                }
            }
        }

        (starting_pos.unwrap(), starting_dir.unwrap())
    }

    fn guard(
        &self,
        matrix: &Vec<Vec<char>>,
        starting_pos: Point,
        starting_dir: Direction,
    ) -> Option<HashSet<(Point, Direction)>> {
        let right = Point::new(0, 1);
        let down = Point::new(1, 0);
        let up = Point::new(-1, 0);
        let left = Point::new(0, -1);

        let mut current_pos = starting_pos;
        let mut current_dir = starting_dir;
        let mut visited_pos: HashSet<(Point, Direction)> = HashSet::new();

        while let Some(current_chr) = self.get_point(matrix, current_pos) {
            if current_chr == '#' {
                if current_dir == Direction::Up {
                    current_pos = current_pos + down + right;
                    current_dir = Direction::Right;
                } else if current_dir == Direction::Down {
                    current_pos = current_pos + up + left;
                    current_dir = Direction::Left;
                } else if current_dir == Direction::Left {
                    current_pos = current_pos + up + right;
                    current_dir = Direction::Up;
                } else if current_dir == Direction::Right {
                    current_pos = current_pos + left + down;
                    current_dir = Direction::Down;
                }
            } else {
                // if the same position is reached such that he is looking at the same direction
                // it's a time loop
                if visited_pos.insert((current_pos, current_dir)) == false {
                    return None;
                }

                current_pos = current_pos
                    + match current_dir {
                        Direction::Left => left,
                        Direction::Up => up,
                        Direction::Down => down,
                        Direction::Right => right,
                    };
            }
        }

        Some(visited_pos)
    }
}

impl Solution for Day6 {
    fn part1(&self) -> String {
        let matrix = self.get_matrix();
        let (starting_pos, starting_dir) = self.get_starting_info();

        self.guard(&matrix, starting_pos, starting_dir)
            .unwrap()
            .iter()
            .unique_by(|f| f.0)
            .collect_vec()
            .len()
            .to_string()
    }

    fn part2(&self) -> String {
        let matrix = self.get_matrix();
        let height = matrix.len();
        let width = matrix[0].len();
        let (starting_pos, starting_dir) = self.get_starting_info();

        let mut timeloop_counter = 0;
        for i in 0..height {
            for j in 0..width {
                if matrix[i][j] == '.' {
                    let mut m = matrix.clone();
                    m[i][j] = '#';

                    if self.guard(&m, starting_pos, starting_dir).is_none() {
                        timeloop_counter += 1;
                    }
                }
            }
        }

        timeloop_counter.to_string()
    }
}
