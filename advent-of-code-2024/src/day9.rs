use advent_of_code_2024::Solution;

pub struct Day9 {
    pub input: String,
}

impl Solution for Day9 {
    fn part1(&self) -> i128 {
        let mut decompressed_string: Vec<String> = vec![];

        let mut current_file_id = 0;
        let mut current_file_type = true; // true: file, false: free space

        let freespace = ".".to_string();

        for chr in self.input.chars() {
            let n: usize = chr.to_digit(10).unwrap().try_into().unwrap();

            let current_character: String = if current_file_type {
                current_file_id.to_string()
            } else {
                freespace.clone()
            };

            decompressed_string.extend(vec![current_character.to_string(); n]);

            current_file_type = !current_file_type;

            if current_file_type {
                current_file_id += 1;
            }
        }

        let mut back_ptr = decompressed_string.len() - 1;

        for i in 0..decompressed_string.len() {
            if decompressed_string[i] != freespace {
                continue;
            }

            while decompressed_string[back_ptr] == freespace && back_ptr > i {
                back_ptr -= 1;
            }
            decompressed_string[i] = decompressed_string[back_ptr].clone();
            decompressed_string[back_ptr] = freespace.clone();
        }

        let mut calculated_checksum: i128 = 0;

        for i in 0..decompressed_string.len() {
            if let Ok(num) = decompressed_string[i].parse::<i128>() {
                calculated_checksum += (i as i128) * num;
            }
        }

        println!("{}", calculated_checksum);
        0
    }

    fn part2(&self) -> i128 {
        1
    }
}
