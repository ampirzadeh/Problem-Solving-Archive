use std::collections::HashSet;

use advent_of_code_2024::{Point, Solution};

pub struct Day6 {
    pub input: String,
}

impl Day6 {
    fn get_matrix(&self) -> Vec<Vec<char>> {
        self.input
            .split("\n")
            .map(|x| x.chars().collect())
            .collect()
    }

    fn get_point(&self, point: Point) -> Option<char> {
        let matrix = self.get_matrix();
        matrix
            .get(point.x as usize)
            .and_then(|f| f.get(point.y as usize))
            .copied()
    }
}

impl Solution for Day6 {
    fn part1(&self) -> i128 {
        let matrix = self.get_matrix();
        let height = matrix.len();
        let width = matrix[0].len();

        let mut starting_pos: Option<Point> = None;
        let mut starting_dir: Option<Point> = None;

        let right = Point::new(0, 1);
        let down = Point::new(1, 0);
        let up = Point::new(-1, 0);
        let left = Point::new(0, -1);

        for i in 0..height {
            for j in 0..width {
                let chr = matrix[i][j];

                if ['<', '>', 'v', '^'].contains(&chr) {
                    starting_pos = Some(Point::new(i as i128, j as i128));

                    starting_dir = match chr {
                        '>' => Some(right),
                        'v' => Some(down),
                        '<' => Some(left),
                        '^' => Some(up),
                        _ => None,
                    }
                }
            }
        }

        let mut visited_pos: HashSet<Point> = HashSet::new();

        let mut current_pos = starting_pos.unwrap();
        let mut current_dir = starting_dir.unwrap();

        while let Some(current_chr) = self.get_point(current_pos) {
            if current_chr == '#' {
                if current_dir == up {
                    current_pos = current_pos + down + right;
                    current_dir = right;
                } else if current_dir == down {
                    current_pos = current_pos + up + left;
                    current_dir = left;
                } else if current_dir == left {
                    current_pos = current_pos + up + right;
                    current_dir = up;
                } else if current_dir == right {
                    current_pos = current_pos + left + down;
                    current_dir = down;
                }
            } else {
                visited_pos.insert(current_pos);
                current_pos = current_pos + current_dir;
            }
        }

        visited_pos.len().try_into().unwrap()
    }

    fn part2(&self) -> i128 {
        0
    }
}
