use std::usize;

use advent_of_code_2024::{Point, Solution};
use itertools::Itertools;
use pathfinding::prelude::bfs;

pub struct Day18 {
    pub input: String,
}

impl Day18 {
    fn get_walls(&self, bytes: usize) -> Vec<Point> {
        let mut walls: Vec<Point> = Vec::new();
        for info in self.input.lines().take(bytes) {
            let (x, y): (i128, i128) = info
                .split(",")
                .map(|f| f.parse::<i128>().unwrap())
                .collect_tuple()
                .unwrap();
            walls.push(Point::new(x, y));
        }

        walls
    }

    fn find_shortest_path(&self, walls: &Vec<Point>, max_x: i128, max_y: i128) -> Option<usize> {
        let start_node = Point::new(0, 0);
        let end_node = Point::new(max_x, max_y);

        bfs(
            &start_node,
            |&p| {
                let mut to_visit = vec![];

                if p.x > 0 {
                    let n = p + Point::new(-1, 0);
                    if walls.contains(&n) == false {
                        to_visit.push(n);
                    }
                }
                if p.y > 0 {
                    let n = p + Point::new(0, -1);
                    if walls.contains(&n) == false {
                        to_visit.push(n);
                    }
                }
                if p.x < max_x {
                    let n = p + Point::new(1, 0);
                    if walls.contains(&n) == false {
                        to_visit.push(n);
                    }
                }
                if p.y < max_y {
                    let n = p + Point::new(0, 1);
                    if walls.contains(&n) == false {
                        to_visit.push(n);
                    }
                }

                to_visit
            },
            |p| *p == end_node,
        )
        .and_then(|f| Some(f.len()))
    }
}

impl Solution for Day18 {
    fn part1(&self) -> i128 {
        let bytes = 1024;
        let max_x = 70;
        let max_y = 70;

        let walls = self.get_walls(bytes);
        self.find_shortest_path(&walls, max_x, max_y).unwrap() as i128 - 1
    }

    fn part2(&self) -> i128 {
        let max_x = 70;
        let max_y = 70;

        let walls = self.get_walls(usize::MAX);

        // binary search the cutoff point
        let mut low = 0;
        let mut high = walls.len();

        while high - low > 1 {
            let middle = (low + high) / 2;
            if self
                .find_shortest_path(&walls.clone().into_iter().take(middle).collect(), max_x, max_y)
                .is_none()
            {
                high = middle;
            } else {
                low = middle;
            }
        }

        println!("{},{}", walls[low].x, walls[low].y);

        0
    }
}
