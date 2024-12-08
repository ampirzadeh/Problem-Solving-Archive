use advent_of_code_2024::Solution;
use itertools::{iproduct, Itertools};
use std::{
    collections::VecDeque,
    iter::{repeat, Filter},
    string,
};

pub struct Day7 {
    pub input: String,
}

impl Day7 {
    fn evaluate_ltr(nums: VecDeque<i64>, operators: VecDeque<char>) -> i64 {
        let mut nums = nums;
        let mut operators = operators;

        let mut res = nums.pop_front().unwrap();

        while let Some(num) = nums.pop_front() {
            match operators.pop_front().unwrap() {
                '+' => res += num,
                '*' => res *= num,
                _ => (),
            }
        }

        res
    }
}

impl Solution for Day7 {
    fn part1(&self) -> i32 {
        let mut valid_sum = 0;

        for line in self.input.split("\n") {
            let (test_value, equation) = line.split_once(": ").unwrap();

            let test_value = test_value.parse::<i64>().unwrap();
            let equation_numbers: Vec<i64> = equation
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            let op = vec!['*', '+'];

            for equation_operators in repeat(op.clone().iter())
                .take(equation_numbers.len() - 1)
                .multi_cartesian_product()
            {
                if test_value
                    == Self::evaluate_ltr(
                        equation_numbers.clone().into(),
                        equation_operators.into_iter().cloned().collect(),
                    )
                {
                    valid_sum += test_value;
                    break;
                }
            }
        }

        println!("{valid_sum}");

        0
    }

    fn part2(&self) -> i32 {
        let mut valid_sum = 0;

        for line in self.input.split("\n") {
            let (test_value, equation) = line.split_once(": ").unwrap();

            let test_value = test_value.parse::<i64>().unwrap();
            let equation_numbers: Vec<i64> = equation
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            let op = vec!['*', '+', '|'];

            for equation_operators in repeat(op.clone().iter())
                .take(equation_numbers.len() - 1)
                .multi_cartesian_product()
            {
                println!("checking {line}");

                let equation_numbers: Vec<i64> = equation_numbers
                    .clone()
                    .into_iter()
                    .zip(equation_operators.clone().into_iter())
                    .clone()
                    .flat_map(|(a, b)| vec![a.to_string(), b.to_string()])
                    .collect::<String>()
                    .replace("|", "")
                    .split(|c| c == '+' || c == '*')
                    .join(" ")
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect();

                if test_value
                    == Self::evaluate_ltr(
                        equation_numbers.clone().into(),
                        equation_operators
                            .into_iter()
                            .cloned()
                            .filter(|c| *c != '|')
                            .collect(),
                    )
                {
                    valid_sum += test_value;
                    break;
                }
            }
        }

        println!("{}", valid_sum);

        0
    }
}
