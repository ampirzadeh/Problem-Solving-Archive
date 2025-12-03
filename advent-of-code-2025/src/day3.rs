use advent_of_code_2025::Solution;

pub struct Day3 {
    pub input: String,
}

impl Day3 {
    fn find_max(digits: Vec<u32>) -> (usize, u32) {
        let mut idx = 0;
        let mut val = 0;

        for i in 0..digits.len() {
            if digits[i] > val {
                val = digits[i];
                idx = i;
            }
        }

        (idx, val)
    }
}

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
        let mut sum = 0;

        for line in self.input.lines() {
            let digits = line
                .chars()
                .map(|f| f.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            let n = 12;

            let mut res = 0u128;
            let mut last_max_idx = 0;
            for i in 1..=n {
                let available_digits = digits[last_max_idx..digits.len() - n + i].to_vec();
                let (max_idx, max_val) = Self::find_max(available_digits);
                last_max_idx += max_idx + 1;
                res = res * 10 + max_val as u128;
            }

            sum += res;
        }

        sum.to_string()
    }
}
