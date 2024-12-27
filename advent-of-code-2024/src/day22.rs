use advent_of_code_2024::Solution;

pub struct Day22 {
    pub input: String,
}

fn mix(a: i128, b: i128) -> i128 {
    a ^ b
}

fn prune(a: i128) -> i128 {
    a % 16777216
}

impl Solution for Day22 {
    fn part1(&self) -> String {
        let mut sum = 0;

        for line in self.input.lines() {
            let mut x: i128 = line.parse().unwrap();
            
            for _ in 0..2000 {
                x = prune(mix(x * 64, x));
                x = prune(mix(x / 32, x));
                x = prune(mix(x * 2048, x));
            }

            sum += x;
        }

        sum.to_string()
    }

    fn part2(&self) -> String {
        todo!()
    }
}
