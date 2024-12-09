use advent_of_code_2024::Solution;
use itertools::Itertools;
use std::{collections::VecDeque, iter::repeat};

#[derive(Clone, PartialEq, Debug)]
enum MyOperators {
    Add,
    Multiply,
    Connect,
}

pub struct Day7 {
    pub input: String,
}

impl Day7 {
    fn concat(a: i128, b: i128) -> i128 {
        a as i128 * 10i128.pow(b.ilog10() + 1) + b as i128
    }

    fn evaluate_ltr(nums: VecDeque<i128>, operators: VecDeque<MyOperators>) -> i128 {
        let mut nums = nums;
        let mut operators = operators;

        let mut res = nums.pop_front().unwrap();

        while let Some(num) = nums.pop_front() {
            match operators.pop_front().unwrap() {
                MyOperators::Add => res += num,
                MyOperators::Multiply => res *= num,
                MyOperators::Connect => res = Self::concat(res, num),
            }
        }

        res
    }

    fn get_line_info(&self, line: &str) -> (i128, Vec<i128>) {
        let (test_value, equation) = line.split_once(": ").unwrap();

        let test_value = test_value.parse::<i128>().unwrap();
        let equation_numbers: Vec<i128> = equation
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        (test_value, equation_numbers)
    }

    fn valid_equation_exists(
        &self,
        test_value: i128,
        equation_numbers: Vec<i128>,
        available_operators: Vec<MyOperators>,
    ) -> bool {
        for equation_operators in repeat(available_operators.clone().iter())
            .take(equation_numbers.len() - 1)
            .multi_cartesian_product()
        {
            if test_value
                == Self::evaluate_ltr(
                    equation_numbers.clone().into(),
                    equation_operators.into_iter().cloned().collect(),
                )
            {
                return true;
            }
        }

        return false;
    }
}

impl Solution for Day7 {
    fn part1(&self) -> i128 {
        let mut valid_sum = 0;

        for line in self.input.split("\n") {
            let (test_value, equation_numbers) = self.get_line_info(line);

            let op = vec![MyOperators::Add, MyOperators::Multiply];

            if self.valid_equation_exists(test_value, equation_numbers, op) {
                valid_sum += test_value;
            }
        }

        valid_sum
    }

    fn part2(&self) -> i128 {
        let mut valid_sum = 0;

        for line in self.input.split("\n") {
            let (test_value, equation_numbers) = self.get_line_info(line);

            let op = vec![
                MyOperators::Add,
                MyOperators::Multiply,
                MyOperators::Connect,
            ];

            if self.valid_equation_exists(test_value, equation_numbers, op) {
                valid_sum += test_value;
            }
        }

        valid_sum
    }
}
