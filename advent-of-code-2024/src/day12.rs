use std::collections::HashMap;

use advent_of_code_2024::{Point, Solution};

pub struct Day12 {
    pub input: String,
}

impl Day12 {
    fn get_matrix(&self) -> Vec<Vec<char>> {
        self.input
            .lines()
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

impl Solution for Day12 {
    fn part1(&self) -> String {
        let mut areas: HashMap<char, i128> = HashMap::new();
        let mut perimeters: HashMap<char, i128> = HashMap::new();

        for (row, roww) in self.get_matrix().iter().enumerate() {
            for (col, coll) in roww.iter().enumerate() {
                areas.entry(*coll).and_modify(|f| *f += 1).or_insert(1);

                let top = self.get_point(Point::new(row as i128 - 1, col as i128));
                if top.is_none() || top.is_some_and(|f| f != *coll) {
                    perimeters.entry(*coll).and_modify(|f| *f += 1).or_insert(1);
                }
                let bottom = self.get_point(Point::new(row as i128 + 1, col as i128));
                if bottom.is_none() || bottom.is_some_and(|f| f != *coll) {
                    perimeters.entry(*coll).and_modify(|f| *f += 1).or_insert(1);
                }
                let left = self.get_point(Point::new(row as i128, col as i128 - 1));
                if left.is_none() || left.is_some_and(|f| f != *coll) {
                    perimeters.entry(*coll).and_modify(|f| *f += 1).or_insert(1);
                }
                let right = self.get_point(Point::new(row as i128, col as i128 + 1));
                if right.is_none() || right.is_some_and(|f| f != *coll) {
                    perimeters.entry(*coll).and_modify(|f| *f += 1).or_insert(1);
                }
            }
        }

        //println!("areas: {:?}\nperimeters: {:?}", areas, perimeters);

        let mut cost = 0;

        for (key, value) in &areas {
            cost += perimeters.get(key).unwrap() * value;
            //println!("cost of {key} is {}", perimeters.get(key).unwrap() * value);
        }
        cost.to_string()
    }

    fn part2(&self) -> String {
        todo!();
    }
}
