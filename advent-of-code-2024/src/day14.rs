use std::collections::HashMap;

use advent_of_code_2024::{Point, Solution};
use regex::{Captures, Regex};

pub struct Day14 {
    pub input: String,
}

impl Day14 {
    fn get_capture_name(&self, cap: &Captures, n: &str) -> i128 {
        cap.name(n).unwrap().as_str().parse().unwrap()
    }
}
impl Solution for Day14 {
    fn part1(&self) -> i128 {
        let lines = self.input.lines();

        let mut robot_position_tracker: HashMap<Point, i128> = HashMap::new();

        let max_height = 103;
        let max_width = 101;
        let seconds_past = 100;

        for line in lines {
            let re = Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
            let cap = re.captures(line).unwrap();

            let pos = Point::new(
                self.get_capture_name(&cap, "px"),
                self.get_capture_name(&cap, "py"),
            );
            let velocity = Point::new(
                self.get_capture_name(&cap, "vx"),
                self.get_capture_name(&cap, "vy"),
            );

            let mut final_pos = pos + velocity * seconds_past;
            final_pos = final_pos % Point::new(max_width, max_height);

            if final_pos.x < 0 {
                final_pos.x = max_width + final_pos.x;
            }
            if final_pos.y < 0 {
                final_pos.y = max_height + final_pos.y;
            }

            robot_position_tracker
                .entry(final_pos)
                .and_modify(|f| *f += 1)
                .or_insert(1);
        }

        let mut top_right = 0;
        let mut top_left = 0;
        let mut bottom_right = 0;
        let mut bottom_left = 0;

        for (pos, rep) in robot_position_tracker {
            if pos.x < max_width / 2 && pos.y > max_height / 2 {
                bottom_left += rep;
            }
            if pos.x > max_width / 2 && pos.y > max_height / 2 {
                bottom_right += rep;
            }
            if pos.x < max_width / 2 && pos.y < max_height / 2 {
                top_left += rep;
            }
            if pos.x > max_width / 2 && pos.y < max_height / 2 {
                top_right += rep;
            }
        }

        bottom_left * bottom_right * top_left * top_right
    }

    fn part2(&self) -> i128 {
        // roses are red
        // violets are blue
        // i have not a single clue
        // of what i should do
        // lol
        0
    }
}
