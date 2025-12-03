use advent_of_code_2025::Solution;

pub struct Day3 {
    pub input: String,
}

impl Day3 {}

impl Solution for Day3 {
    fn part1(&self) -> String {
        let mut sum = 0;

        for line in self.input.lines() {
            let digits = line
                .chars()
                .map(|f| f.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            let mut max_combo = vec![0, 0];
            for i in 0..digits.len() {
                if digits[i] > max_combo[1] {
                    max_combo[1] = digits[i];
                }
                if digits[i] > max_combo[0] && i != digits.len() - 1 {
                    max_combo[0] = digits[i];
                    max_combo[1] = 0;
                }
            }

            sum += max_combo[0] * 10 + max_combo[1];
        }

        sum.to_string()
    }

    fn part2(&self) -> String {
        return "unimplemented".to_string();
    }
}
