use advent_of_code_2024::Solution;
use itertools::Itertools;
use std::collections::VecDeque;

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

            let mut available_operators = vec!['*'; equation_numbers.len() - 1];
            available_operators.extend(vec!['+'; equation_numbers.len() - 1]);

            println!("checking {line}");

            for equation_operators in available_operators
                .into_iter()
                .permutations(equation_numbers.len() - 1)
                .unique()
            {
                if test_value
                    == Self::evaluate_ltr(
                        equation_numbers.clone().into(),
                        equation_operators.into(),
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

    fn part2(&self) -> i32 {
        0
    }
}
