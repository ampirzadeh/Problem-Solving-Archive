use advent_of_code_2025::Solution;

pub struct Day7 {
    pub input: String,
}

impl Day7 {}

impl Solution for Day7 {
    fn part1(&self) -> String {
        let mut count = 0;

        let mut lines = self.input.lines();
        let first_line = lines.next().unwrap();

        let mut beam_positions: Vec<bool> = vec![false; first_line.len()]; // false for empty space, true for beam
        beam_positions[first_line.find('S').unwrap()] = true; // beam originates from S

        for line in lines {
            for (i, chr) in line.chars().enumerate() {
                if chr == '^' && beam_positions[i] == true {
                    // a beam is hitting a splitter
                    count += 1;

                    beam_positions[i] = false; // the beam is stopped
                    beam_positions[i - 1] = true; // left split
                    beam_positions[i + 1] = true; // right split
                }
            }
        }

        count.to_string()
    }

    fn part2(&self) -> String {
        let mut lines = self.input.lines();
        let first_line = lines.next().unwrap();

        let mut timeline_count: Vec<u128> = vec![0; first_line.len()]; // the number of ways to get to each position
        timeline_count[first_line.find('S').unwrap()] = 1; // beam originates from S

        for line in lines {
            let mut new_timeline_count: Vec<u128> = vec![0; line.len()];

            for (index, count) in timeline_count.iter().enumerate() {
                if line.chars().nth(index).unwrap() == '^' {
                    new_timeline_count[index - 1] += count;
                    new_timeline_count[index + 1] += count;
                } else {
                    new_timeline_count[index] += count;
                }
            }
            timeline_count = new_timeline_count;
        }

        timeline_count.iter().sum::<u128>().to_string()
    }
}
