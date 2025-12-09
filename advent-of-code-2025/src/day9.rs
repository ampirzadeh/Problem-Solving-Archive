use std::collections::HashSet;

use advent_of_code_2025::{Solution, Vector2d};

pub struct Day9 {
    pub input: String,
}

type Rectangle = (Vector2d, Vector2d, i64);

impl Day9 {
    fn get_red_tiles(input: &String) -> Vec<Vector2d> {
        let mut out = vec![];

        for line in input.lines() {
            let data = line.split_once(',').unwrap();
            let x = data.0.parse().unwrap();
            let y = data.1.parse().unwrap();
            out.push(Vector2d { x, y });
        }

        out
    }

    fn get_rectangles(red_tiles: &Vec<Vector2d>) -> Vec<Rectangle> {
        let mut all_rectangles: Vec<Rectangle> = vec![]; // Corner, opposite corner, area

        for (i, p1) in red_tiles.iter().enumerate() {
            for p2 in red_tiles[i + 1..].iter() {
                let area = (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1);
                all_rectangles.push((p1.clone(), p2.clone(), area as i64));
            }
        }

        all_rectangles
    }
}

impl Solution for Day9 {
    fn part1(&self) -> String {
        let red_tiles = Self::get_red_tiles(&self.input);
        let rectangles = Self::get_rectangles(&red_tiles);

        let max_rectangle = rectangles.iter().max_by_key(|x| x.2).unwrap();

        max_rectangle.2.to_string()
    }

    fn part2(&self) -> String {
        let red_tiles = Self::get_red_tiles(&self.input);

        let mut all_rectangles = Self::get_rectangles(&red_tiles);
        all_rectangles.sort_by_key(|(_, _, area)| -*area); // sort them by area desc

        let mut perimeter_points: HashSet<Vector2d> = HashSet::new();

        let n = red_tiles.len();
        for i in 0..n {
            let p1 = red_tiles[i];
            let p2 = red_tiles[(i + 1) % n];

            let (low_x, high_x) = if p1.x < p2.x {
                (p1.x, p2.x)
            } else {
                (p2.x, p1.x)
            };
            let (low_y, high_y) = if p1.y < p2.y {
                (p1.y, p2.y)
            } else {
                (p2.y, p1.y)
            };

            for x in low_x..high_x + 1 {
                for y in low_y..high_y + 1 {
                    perimeter_points.insert(Vector2d { x, y });
                }
            }
        }

        for (p1, p2, area) in all_rectangles {
            let (low_x, high_x) = if p1.x < p2.x {
                (p1.x, p2.x)
            } else {
                (p2.x, p1.x)
            };
            let (low_y, high_y) = if p1.y < p2.y {
                (p1.y, p2.y)
            } else {
                (p2.y, p1.y)
            };

            if perimeter_points.iter().any(|point| {
                let x = point.x < high_x && point.x > low_x;
                let y = point.y < high_y && point.y > low_y;
                return x && y;
            }) {
                continue;
            }

            return area.to_string();
        }

        0.to_string()
    }
}
