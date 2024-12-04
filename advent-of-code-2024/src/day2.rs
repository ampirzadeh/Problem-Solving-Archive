use advent_of_code_2024::Solution;

pub struct Day2 {
    pub input: String,
}

impl Day2 {
    fn is_valid_report(report: &Vec<i32>) -> bool {
        let mut differences: Vec<i32> = Vec::new();

        for i in 1..report.len() {
            let first = report[i - 1];
            let second = report[i];

            differences.push(first - second);
        }

        differences.iter().all(|&x| x.abs() >= 1 && x.abs() <= 3)
            && (differences.iter().all(|&x| x > 0) || differences.iter().all(|&x| x < 0))
    }
}

impl Solution for Day2 {
    fn part1(&self) -> i32 {
        let mut valid_counter = 0;

        for line in self.input.split("\n") {
            let report = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            if Self::is_valid_report(&report) {
                valid_counter += 1;
            }
        }

        valid_counter
    }

    fn part2(&self) -> i32 {
        let mut valid_counter = 0;

        for line in self.input.split("\n") {
            let report: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            if Self::is_valid_report(&report) {
                valid_counter += 1;
                continue;
            }

            let mut subreports: Vec<Vec<i32>> = Vec::new();
            for i in 0..report.len() {
                let subset: Vec<i32> = report
                    .iter()
                    .enumerate()
                    .filter(|&(index, _)| index != i)
                    .map(|(_, &value)| value)
                    .collect();

                subreports.push(subset);
            }
            for sublevel in subreports {
                if Self::is_valid_report(&sublevel) {
                    valid_counter += 1;
                    break;
                }
            }
        }

        valid_counter
    }
}
