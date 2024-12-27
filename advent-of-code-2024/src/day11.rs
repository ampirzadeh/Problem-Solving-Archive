use std::collections::HashMap;

use advent_of_code_2024::Solution;

pub struct Day11 {
    pub input: String,
}

struct Blinker {
    stones: HashMap<i128, i128>,
}

impl Blinker {
    fn get_num_length(&self, a: i128) -> i128 {
        (a.ilog10() + 1).into()
    }

    fn split_in_half(&self, a: i128) -> (i128, i128) {
        let num_len = self.get_num_length(a);
        let b = 10i128.pow((num_len / 2) as u32);

        (a / b, a % b)
    }
}

impl Iterator for Blinker {
    type Item = HashMap<i128, i128>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_turn: HashMap<i128, i128> = HashMap::new();

        for (&num, &repeats) in self.stones.iter() {
            if num == 0 {
                next_turn
                    .entry(1)
                    .and_modify(|f| *f += repeats)
                    .or_insert(repeats);
            } else if self.get_num_length(num) % 2 == 0 {
                let (a, b) = self.split_in_half(num);

                next_turn
                    .entry(a)
                    .and_modify(|f| *f += repeats)
                    .or_insert(repeats);
                next_turn
                    .entry(b)
                    .and_modify(|f| *f += repeats)
                    .or_insert(repeats);
            } else {
                next_turn
                    .entry(num * 2024)
                    .and_modify(|f| *f += repeats)
                    .or_insert(repeats);
            }
        }

        self.stones = next_turn.clone();
        Some(next_turn)
    }
}

impl Day11 {
    fn get_frequency_map(&self) -> HashMap<i128, i128> {
        let mut frequence_counter: HashMap<i128, i128> = HashMap::new();

        let nums: Vec<i128> = self
            .input
            .split_whitespace()
            .map(|s| s.parse::<i128>().unwrap())
            .collect();

        for num in nums {
            *frequence_counter.entry(num).or_insert(0) += 1;
        }
        frequence_counter
    }
}

impl Solution for Day11 {
    // at first i implemented a vector approach instead of a hashmap for part 1
    // after getting stuck at part 2 for a while i thought there must be a trick to it
    // and there is!
    // the number of unique elements in the list (simply .iter().unique()) grew to be 54 and
    // then stopped increasing
    // the mathematical proof is quite fun and simple once you know it happens
    // so a hashmap of 54 keys is the way to go

    fn part1(&self) -> String {
        let mut blinker = Blinker {
            stones: self.get_frequency_map(),
        };
        let stones = blinker.nth(24).unwrap();
        stones.into_iter().map(|(_, repeats)| repeats).sum::<i128>().to_string()
    }

    fn part2(&self) -> String {
        let mut blinker = Blinker {
            stones: self.get_frequency_map(),
        };

        let stones = blinker.nth(74).unwrap();
        stones.into_iter().map(|(_, repeats)| repeats).sum::<i128>().to_string()
    }
}
