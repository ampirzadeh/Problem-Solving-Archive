use advent_of_code_2025::Solution;

pub struct Day5 {
    pub input: String,
}

impl Day5 {
    // Reuse from day 2
    fn get_ranges(inp: &str) -> Vec<(u128, u128)> {
        let mut ids = vec![];

        for num_range in inp.lines() {
            let data = num_range
                .split("-")
                .map(|s| s.parse::<u128>().unwrap())
                .collect::<Vec<u128>>();
            let from = data.get(0).unwrap().to_owned();
            let to = data.get(1).unwrap().to_owned();

            ids.push((from, to));
        }

        ids
    }
}

impl Solution for Day5 {
    fn part1(&self) -> String {
        let mut fresh = 0;

        let tmp = self.input.split("\n\n").collect::<Vec<&str>>();
        let fresh_id_ranges = Self::get_ranges(tmp[0]);

        'a: for line in tmp[1].lines() {
            let id = line.parse::<u128>().unwrap();
            for (from, to) in fresh_id_ranges.iter() {
                if id >= *from && id <= *to {
                    fresh += 1;
                    continue 'a;
                }
            }
        }

        fresh.to_string()
    }

    fn part2(&self) -> String {
        let mut total_valid_ids = 0;

        let mut id_ranges = Self::get_ranges(self.input.split("\n\n").collect::<Vec<&str>>()[0]);

        id_ranges.sort_by(|a, b| a.0.cmp(&b.0)); // Sort asc by the lower bounds
        let mut merged_id_ranges: Vec<(u128, u128)> = vec![id_ranges.remove(0)]; // Populate it initially

        for current_range in id_ranges {
            let last_added = merged_id_ranges.last_mut().unwrap();

            if current_range.0 <= last_added.1 {
                *last_added = (last_added.0, current_range.1.max(last_added.1));
            } else {
                merged_id_ranges.push(current_range);
            }
        }

        for id_range in merged_id_ranges {
            total_valid_ids += id_range.1 - id_range.0 + 1;
        }

       total_valid_ids.to_string()
    }
}
