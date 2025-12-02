// I made a point of not using strings and just using the numbers
// and logs directly to manipulate repeated sequences!

use advent_of_code_2025::Solution;

pub struct Day2 {
    pub input: String,
}

impl Day2 {
    fn get_ranges(inp: &String) -> Vec<u128> {
        let mut result = vec![];

        for num_range in inp.split(",") {
            let data = num_range
                .split("-")
                .map(|s| s.parse::<u128>().unwrap())
                .collect::<Vec<u128>>();
            let from = data.get(0).unwrap().to_owned();
            let to = data.get(1).unwrap().to_owned();

            for i in from..=to {
                result.push(i);
            }
        }

        result
    }

    fn is_invalid_id_part_1(id: u128) -> bool {
        let n_of_digits = id.checked_ilog10().unwrap_or(0) + 1;

        // My initial solution:
        // if n_of_digits % 2 == 1 {
        //     return false;
        // }
        // let right_half = id / 10u128.pow(n_of_digits / 2);
        // let left_half = id % 10u128.pow(n_of_digits / 2);
        // return right_half == left_half;

        let repeated = Self::repeat(id % 10u128.pow(n_of_digits / 2), 2); // repeat half of id
        return repeated == id;
    }

    fn repeat(seq: u128, n: u32) -> u128 {
        let mut result = seq;
        let n_of_digits = seq.checked_ilog10().unwrap_or(0) + 1;

        for _ in 1..n {
            result *= 10u128.pow(n_of_digits);
            result += seq;
        }
        result
    }

    fn is_invalid_id_part_2(id: u128) -> bool {
        let n_of_digits = id.checked_ilog10().unwrap_or(0) + 1;
        for i in 1..(n_of_digits / 2 + 1) {
            if n_of_digits % i != 0 {
                continue;
            }
            let repeated = Self::repeat(id % 10u128.pow(i), n_of_digits / i);
            if id == repeated {
                return true;
            }
        }
        return false;
    }
}

impl Solution for Day2 {
    fn part1(&self) -> String {
        let mut sum = 0;

        for i in Self::get_ranges(&self.input) {
            if Self::is_invalid_id_part_1(i) {
                sum += i;
            }
        }

        sum.to_string()
    }

    fn part2(&self) -> String {
        let mut sum = 0;

        for i in Self::get_ranges(&self.input) {
            if Self::is_invalid_id_part_2(i) {
                sum += i;
            }
        }

        sum.to_string()
    }
}
