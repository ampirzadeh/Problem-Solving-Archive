use advent_of_code_2024::Solution;
use regex::Regex;

pub struct Day13 {
    pub input: String,
}

impl Day13 {
    fn solve_linear_equations(
        &self,
        a1: f64,
        b1: f64,
        c1: f64,
        a2: f64,
        b2: f64,
        c2: f64,
    ) -> Option<(f64, f64)> {
        let det = a1 * b2 - a2 * b1;

        if det == 0.0 {
            return None; // No unique solution
        }

        let det_x = c1 * b2 - c2 * b1;
        let det_y = a1 * c2 - a2 * c1;

        let x = det_x / det;
        let y = det_y / det;

        Some((x, y))
    }
}

impl Solution for Day13 {
    fn part1(&self) -> String {
        let mut lines = self.input.lines();

        let button_reg = Regex::new(r"X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
        let prize_reg = Regex::new(r"X\=(?<x>\d+), Y\=(?<y>\d+)").unwrap();

        let mut sum: f64 = 0.;
        loop {
            let button_a = lines.next().unwrap();
            let button_b = lines.next().unwrap();
            let prize = lines.next().unwrap();

            let cap = button_reg.captures(button_a).unwrap();
            let ax = cap.name("x").unwrap().as_str().parse::<f64>().unwrap();
            let ay = cap.name("y").unwrap().as_str().parse::<f64>().unwrap();

            let cap = button_reg.captures(button_b).unwrap();
            let bx = cap.name("x").unwrap().as_str().parse::<f64>().unwrap();
            let by = cap.name("y").unwrap().as_str().parse::<f64>().unwrap();

            let cap = prize_reg.captures(prize).unwrap();
            let x_goal = cap.name("x").unwrap().as_str().parse::<f64>().unwrap();
            let y_goal = cap.name("y").unwrap().as_str().parse::<f64>().unwrap();

            let Some((a_press_count, b_press_count)) =
                self.solve_linear_equations(ax, bx, x_goal, ay, by, y_goal)
            else {
                continue;
            };

            if a_press_count % 1.0 == 0.0 && b_press_count % 1.0 == 0.0 {
                sum += 3.0 * a_press_count + b_press_count;
            }

            let Some(_) = lines.next() else {
                break;
            };
        }

        sum.to_string()
    }

    fn part2(&self) -> String {
        let mut lines = self.input.lines();

        let button_reg = Regex::new(r"X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
        let prize_reg = Regex::new(r"X\=(?<x>\d+), Y\=(?<y>\d+)").unwrap();

        let mut sum: f64 = 0.;
        loop {
            let button_a = lines.next().unwrap();
            let button_b = lines.next().unwrap();
            let prize = lines.next().unwrap();

            let cap = button_reg.captures(button_a).unwrap();
            let ax = cap.name("x").unwrap().as_str().parse::<f64>().unwrap();
            let ay = cap.name("y").unwrap().as_str().parse::<f64>().unwrap();

            let cap = button_reg.captures(button_b).unwrap();
            let bx = cap.name("x").unwrap().as_str().parse::<f64>().unwrap();
            let by = cap.name("y").unwrap().as_str().parse::<f64>().unwrap();

            let cap = prize_reg.captures(prize).unwrap();
            let x_goal = cap.name("x").unwrap().as_str().parse::<f64>().unwrap();
            let y_goal = cap.name("y").unwrap().as_str().parse::<f64>().unwrap();

            let Some((a_press_count, b_press_count)) = self.solve_linear_equations(
                ax,
                bx,
                10000000000000.0 + x_goal,
                ay,
                by,
                10000000000000.0 + y_goal,
            ) else {
                continue;
            };

            if a_press_count % 1.0 == 0.0 && b_press_count % 1.0 == 0.0 {
                sum += 3.0 * a_press_count + b_press_count;
            }

            let Some(_) = lines.next() else {
                break;
            };
        }

        sum.to_string()
    }
}
