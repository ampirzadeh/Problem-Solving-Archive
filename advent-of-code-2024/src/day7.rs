use advent_of_code_2024::Solution;
use std::i128;

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
        equation_numbers: &Vec<i128>,
        available_operators: &Vec<MyOperators>,
    ) -> bool {
        let mut equation_numbers = equation_numbers.clone();

        if let Some(last_number) = equation_numbers.pop() {
            if available_operators.contains(&MyOperators::Multiply) {
                if test_value % last_number == 0 {
                    if self.valid_equation_exists(
                        test_value / last_number,
                        &equation_numbers,
                        available_operators,
                    ) {
                        return true;
                    }
                }
            }

            if available_operators.contains(&MyOperators::Add) {
                if test_value - last_number >= 0 {
                    if self.valid_equation_exists(
                        test_value - last_number,
                        &equation_numbers,
                        available_operators,
                    ) {
                        return true;
                    }
                }
            }

            if available_operators.contains(&MyOperators::Connect) {
                let last_number_magnitute = 10i128.pow(last_number.ilog10() + 1);

                if test_value % last_number_magnitute == last_number {
                    if self.valid_equation_exists(
                        test_value / last_number_magnitute,
                        &equation_numbers,
                        available_operators,
                    ) {
                        return true;
                    }
                }
            }

            return false;
        } else {
            return test_value == 0;
        }
    }
}

impl Solution for Day7 {
    fn part1(&self) -> String {
        let mut valid_sum = 0;

        for line in self.input.lines() {
            let (test_value, equation_numbers) = self.get_line_info(line);

            let op = vec![MyOperators::Add, MyOperators::Multiply];

            if self.valid_equation_exists(test_value, &equation_numbers, &op) {
                valid_sum += test_value;
            }
        }

        valid_sum.to_string()
    }

    fn part2(&self) -> String {
        let mut valid_sum = 0;

        for line in self.input.lines() {
            let (test_value, equation_numbers) = self.get_line_info(line);

            let op = vec![
                MyOperators::Add,
                MyOperators::Multiply,
                MyOperators::Connect,
            ];

            if self.valid_equation_exists(test_value, &equation_numbers, &op) {
                valid_sum += test_value;
            }
        }

        valid_sum.to_string()
    }
}
