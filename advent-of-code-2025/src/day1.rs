use advent_of_code_2025::Solution;

pub struct Day1 {
    pub input: String,
}

impl Day1 {}

impl Solution for Day1 {
    fn part1(&self) -> String {
        let mut zero_counter = 0;
        let mut current_pos = 50;

        for line in self.input.lines() {
            let movement_amount = line[1..].parse::<i32>().unwrap();

            if line.starts_with("R") {
                current_pos += movement_amount;
            } else {
                current_pos -= movement_amount;
            };
            current_pos = current_pos % 100;
            if current_pos % 100 == 0 {
                current_pos = 0;
                zero_counter += 1;
            }
        }

        zero_counter.to_string()
    }

    fn part2(&self) -> String {
        let mut zero_counter = 0;
        let mut current_pos = 50;

        for line in self.input.lines() {
            let movement_amount = line[1..].parse::<i32>().unwrap();
            if line.starts_with("R") {
                current_pos += movement_amount;
                zero_counter += current_pos / 100;
                current_pos = current_pos % 100;
                // println!(
                //     "move: +{} current_pos: {} zero_counter: {}",
                //     movement_amount, current_pos, zero_counter
                // );
            } else {
                if current_pos == 0 {
                    if movement_amount <= 100 {
                        current_pos = 100 - movement_amount;
                    } else {
                        current_pos += movement_amount;
                        while current_pos > 100 {
                            current_pos -= 100;
                            zero_counter += 1;
                        }
                        current_pos = 100 - current_pos;
                    }
                } else {
                    current_pos -= movement_amount;
                    while current_pos < 0 {
                        current_pos += 100;
                        zero_counter += 1;
                    }
                }
                if current_pos == 0 {
                    zero_counter += 1;
                }

                // println!(
                //     "move: -{} current_pos: {} zero_counter: {}",
                //     movement_amount, current_pos, zero_counter
                // );
            };
        }

        zero_counter.to_string()
    }
}
