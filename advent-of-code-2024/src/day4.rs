use advent_of_code_2024::Solution;

pub struct Day4 {
    pub input: String,
}

impl Solution for Day4 {
    fn part1(&self) -> i128 {
        let rows: Vec<Vec<char>> = self
            .input
            .split("\n")
            .map(|x| x.chars().collect())
            .collect();
        let height = rows.len();
        let width = rows[0].len();

        let mut diag_down = String::from("");
        for row_idx in 0..(height - 3) {
            for col_idx in 0..(width - 3) {
                diag_down.push_str(
                    format!(
                        "{}{}{}{} ",
                        rows[row_idx][col_idx],
                        rows[row_idx + 1][col_idx + 1],
                        rows[row_idx + 2][col_idx + 2],
                        rows[row_idx + 3][col_idx + 3]
                    )
                    .as_str(),
                );
            }
        }

        let mut diag_up = String::from("");
        for row_idx in 3..height {
            for col_idx in 0..(width - 3) {
                diag_up.push_str(
                    format!(
                        "{}{}{}{} ",
                        rows[row_idx][col_idx],
                        rows[row_idx - 1][col_idx + 1],
                        rows[row_idx - 2][col_idx + 2],
                        rows[row_idx - 3][col_idx + 3]
                    )
                    .as_str(),
                );
            }
        }

        let mut vertical = String::from("");
        for row_idx in 0..(height - 3) {
            for col_idx in 0..width {
                vertical.push_str(
                    format!(
                        "{}{}{}{} ",
                        rows[row_idx][col_idx],
                        rows[row_idx + 1][col_idx],
                        rows[row_idx + 2][col_idx],
                        rows[row_idx + 3][col_idx]
                    )
                    .as_str(),
                );
            }
        }

        let mut counter = 0;
        counter += vertical.matches("XMAS").count();
        counter += vertical.matches("SAMX").count();
        counter += diag_up.matches("SAMX").count();
        counter += diag_up.matches("XMAS").count();
        counter += diag_down.matches("XMAS").count();
        counter += diag_down.matches("SAMX").count();
        counter += self.input.matches("XMAS").count();
        counter += self.input.matches("SAMX").count();
        counter.try_into().unwrap()
    }

    fn part2(&self) -> i128 {
        let rows: Vec<Vec<char>> = self
            .input
            .split("\n")
            .map(|x| x.chars().collect())
            .collect();
        let height = rows.len();
        let width = rows[0].len();

        let mut counter = 0;
        for i in 1..(height - 1) {
            for j in 1..(width - 1) {
                if rows[i][j] == 'A' {
                    let bottom_right_diag = format!("{}{}", rows[i - 1][j - 1], rows[i + 1][j + 1]);
                    let top_right_diag = format!("{}{}", rows[i - 1][j + 1], rows[i + 1][j - 1]);

                    if bottom_right_diag.contains("M")
                        && bottom_right_diag.contains("S")
                        && top_right_diag.contains("M")
                        && top_right_diag.contains("S")
                    {
                        counter += 1;
                    }
                }
            }
        }

        counter.try_into().unwrap()
    }
}
