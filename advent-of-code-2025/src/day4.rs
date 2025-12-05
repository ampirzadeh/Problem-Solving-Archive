use advent_of_code_2025::Solution;

pub struct Day4 {
    pub input: String,
}

impl Day4 {}

impl Solution for Day4 {
    fn part1(&self) -> String {
        let mut res = 0;

        let grid = self
            .input
            .lines()
            .map(|f| f.chars().collect())
            .collect::<Vec<Vec<char>>>();

        for row_idx in 0..grid.len() {
            for col_idx in 0..grid[row_idx].len() {
                if grid[row_idx][col_idx] == '.' {
                    continue;
                }

                let mut paper_roll_count = 0;

                let row_idx = row_idx as i32;
                let col_idx = col_idx as i32;
                let positions = [
                    (row_idx - 1, col_idx),
                    (row_idx + 1, col_idx),
                    (row_idx, col_idx - 1),
                    (row_idx, col_idx + 1),
                    (row_idx - 1, col_idx - 1),
                    (row_idx - 1, col_idx + 1),
                    (row_idx + 1, col_idx - 1),
                    (row_idx + 1, col_idx + 1),
                ];

                for pos in positions {
                    if grid
                        .get(pos.0 as usize)
                        .is_some_and(|r| r.get(pos.1 as usize).is_some_and(|c| *c == '@'))
                    {
                        paper_roll_count += 1;
                    }
                }

                if paper_roll_count < 4 {
                    res += 1;
                }
            }
        }

        res.to_string()
    }

    fn part2(&self) -> String {
        let mut res = 0;

        let mut grid = self
            .input
            .lines()
            .map(|f| f.chars().collect())
            .collect::<Vec<Vec<char>>>();

        loop {
            let mut removed: Vec<(usize, usize)> = vec![];

            for row_idx in 0..grid.len() {
                for col_idx in 0..grid[row_idx].len() {
                    if grid[row_idx][col_idx] == '.' {
                        continue;
                    }

                    let mut paper_roll_count = 0;

                    let row_idx = row_idx as i32;
                    let col_idx = col_idx as i32;
                    let positions = [
                        (row_idx - 1, col_idx),
                        (row_idx + 1, col_idx),
                        (row_idx, col_idx - 1),
                        (row_idx, col_idx + 1),
                        (row_idx - 1, col_idx - 1),
                        (row_idx - 1, col_idx + 1),
                        (row_idx + 1, col_idx - 1),
                        (row_idx + 1, col_idx + 1),
                    ];

                    for pos in positions {
                        if grid
                            .get(pos.0 as usize)
                            .is_some_and(|r| r.get(pos.1 as usize).is_some_and(|c| *c == '@'))
                        {
                            paper_roll_count += 1;
                        }
                    }

                    if paper_roll_count < 4 {
                        removed.push((row_idx as usize, col_idx as usize));
                        res += 1;
                    }
                }
            }
            if removed.len() == 0 {
                break;
            }
            for r in removed {
                grid[r.0][r.1] = '.';
            }
        }

        res.to_string()
    }
}
