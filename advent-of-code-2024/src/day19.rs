use std::collections::HashMap;

use advent_of_code_2024::Solution;

pub struct Day19 {
    pub input: String,
}

type Cache<'a> = HashMap<&'a str, i128>;
struct Onsen {
    towels: Vec<String>,
}
impl Onsen {
    fn arrange_count(&self, pattern: &str) -> i128 {
        let mut c: Cache = Cache::new();
        self.match_count(pattern, &mut c)
    }

    fn match_count<'a>(&self, pattern: &'a str, cache: &mut Cache<'a>) -> i128 {
        if !cache.contains_key(pattern) {
            let a = match pattern {
                "" => 1,
                _ => self
                    .towels
                    .iter()
                    .filter(|f| pattern.starts_with(*f))
                    .map(|f| self.match_count(&pattern[f.len()..], cache))
                    .sum(),
            };
            cache.insert(pattern, a);
        }

        cache[pattern]
    }
}

impl Solution for Day19 {
    fn part1(&self) -> i128 {
        let mut lines = self.input.lines();

        let towels: Vec<String> = lines
            .next()
            .unwrap()
            .split(", ")
            .map(|f| f.to_string())
            .collect();
        lines.next();

        let o = Onsen { towels };
        let mut valid_pattern_count = 0;
        for line in lines {
            if o.arrange_count(line) > 0 {
                valid_pattern_count += 1;
            }
        }

        valid_pattern_count
    }

    fn part2(&self) -> i128 {
        let mut lines = self.input.lines();

        let towels: Vec<String> = lines
            .next()
            .unwrap()
            .split(", ")
            .map(|f| f.to_string())
            .collect();
        lines.next();

        let o = Onsen { towels };
        let mut valid_pattern_count = 0;
        for line in lines {
            valid_pattern_count += o.arrange_count(line);
        }

        valid_pattern_count
    }
}
