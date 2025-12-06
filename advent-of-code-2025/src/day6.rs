// Be VERY careful of the input file spaces. Your IDE or clipboard might change the whilespaces and cause the input to be incorrect
// I suggest you download your input file using curl or wget or from your browser
//
// An improvement I would want to do is to avoid loading the file into memory and instead read it line by line
// The implementation of detecting columns of whitespace would be a bit clunky hence why I didn't do it
// but it would allow for larger files!

use advent_of_code_2025::Solution;

pub struct Day6 {
    pub input: String,
}

#[derive(Debug)]
enum Operations {
    Addition,
    Multiplication,
}

type NumsAndOps = Vec<(Vec<u128>, Operations)>; // For clarity

impl Day6 {
    fn perform_operations(nums_and_ops: NumsAndOps) -> u128 {
        nums_and_ops
            .iter()
            .map(|(nums, op)| match op {
                Operations::Addition => nums.iter().sum::<u128>(),
                Operations::Multiplication => nums.iter().product::<u128>(),
            })
            .sum()
    }
}

impl Solution for Day6 {
    fn part1(&self) -> String {
        let mut nums_and_ops: NumsAndOps = vec![];
        let mut lines: Vec<&str> = self.input.lines().collect();

        lines // Remove the final line containing the operators and parse them
            .remove(lines.len() - 1)
            .split_whitespace()
            .for_each(|op| {
                nums_and_ops.push((
                    vec![],
                    match op {
                        "+" => Operations::Addition,
                        "*" => Operations::Multiplication,
                        _ => unreachable!(),
                    },
                ))
            });

        for line in lines {
            let nums: Vec<u128> = line
                .split_whitespace()
                .map(|s| s.parse::<u128>().unwrap())
                .collect();

            for (i, num) in nums.iter().enumerate() {
                nums_and_ops[i].0.push(num.to_owned());
            }
        }

        Self::perform_operations(nums_and_ops).to_string()
    }

    fn part2(&self) -> String {
        let mut lines: Vec<&str> = self.input.lines().collect();
        let mut nums_and_ops: NumsAndOps = vec![]; // inital

        lines // Remove the final line containing the operators and parse them
            .remove(lines.len() - 1)
            .split_whitespace()
            .for_each(|op| {
                nums_and_ops.push((
                    vec![],
                    match op {
                        "+" => Operations::Addition,
                        "*" => Operations::Multiplication,
                        _ => unreachable!(),
                    },
                ))
            });

        let mut problem_idx = 0;

        // Traverse the matrix column by column
        for col_idx in 0..lines[0].len() {
            let mut vertical_num = 0u128;

            for row_idx in 0..lines.len() {
                let chr = lines[row_idx].chars().nth(col_idx).unwrap();

                if let Some(digit) = chr.to_digit(10) {
                    vertical_num *= 10;
                    vertical_num += digit as u128;
                }
            }
            // a vertical line of spaces causes num to be 0
            if vertical_num == 0 {
                problem_idx += 1; // go to the next problem (the next stack of numbers)
            } else {
                nums_and_ops[problem_idx].0.push(vertical_num);
            }
        }

        Self::perform_operations(nums_and_ops).to_string()
    }
}
