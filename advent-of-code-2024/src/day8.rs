use std::{
    char,
    collections::{HashMap, HashSet},
};

use advent_of_code_2024::{Point, Solution};

pub struct Day8 {
    pub input: String,
}

impl Day8 {
    fn get_stations(&self, matrix: Vec<Vec<char>>) -> HashMap<char, Vec<Point>> {
        let height = matrix.len();
        let width = matrix[0].len();

        let mut frequency_station_mapper: HashMap<char, Vec<Point>> = HashMap::new();
        for i in 0..height {
            for j in 0..width {
                let chr = matrix[i][j];
                if chr == '.' {
                    continue;
                }

                let pos = Point::new(i as i128, j as i128);
                frequency_station_mapper
                    .entry(chr)
                    .and_modify(|x| {
                        x.push(pos);
                    })
                    .or_insert(vec![pos]);
            }
        }
        frequency_station_mapper
    }

    fn get_antinodes(&self, stations: Vec<Point>, max_height: i128, max_width: i128) -> Vec<Point> {
        let mut res = vec![];

        for i in 0..stations.len() {
            for j in (i + 1)..stations.len() {
                let diff = stations[i] - stations[j];

                let a = stations[i] + diff;
                if !(a.x >= max_height || a.x < 0 || a.y >= max_width || a.y < 0) {
                    res.push(a);
                }

                let b = stations[j] - diff;
                if !(b.x >= max_height || b.x < 0 || b.y >= max_width || b.y < 0) {
                    res.push(b);
                }
            }
        }

        res
    }

    fn get_antinodes_with_repeats(
        &self,
        stations: Vec<Point>,
        max_height: i128,
        max_width: i128,
    ) -> Vec<Point> {
        let mut res = vec![];

        for i in 0..stations.len() {
            for j in (i + 1)..stations.len() {
                let diff = stations[i] - stations[j];

                let mut diff_coefficient = 1;
                loop {
                    let a = stations[i] + diff * diff_coefficient;
                    if a.x >= max_height || a.x < 0 {
                        break;
                    }
                    if a.y >= max_width || a.y < 0 {
                        break;
                    }

                    res.push(a);
                    diff_coefficient += 1;
                }

                diff_coefficient = 1;
                loop {
                    let a = stations[j] - diff * diff_coefficient;
                    if a.x >= max_height || a.x < 0 {
                        break;
                    }
                    if a.y >= max_width || a.y < 0 {
                        break;
                    }

                    res.push(a);
                    diff_coefficient += 1;
                }
            }
        }

        res.extend(stations);

        res
    }
}

impl Solution for Day8 {
    fn part1(&self) -> String {
        let matrix: Vec<Vec<char>> = self.input.lines().map(|x| x.chars().collect()).collect();
        let height = matrix.len() as i128;
        let width = matrix[0].len() as i128;

        let mut antinode_locations: HashSet<Point> = HashSet::new();

        let frequency_station_mapper = self.get_stations(matrix);

        for (_, station_coordinates) in frequency_station_mapper.iter() {
            for f in self.get_antinodes(station_coordinates.to_vec(), height, width) {
                antinode_locations.insert(f);
            }
        }

        antinode_locations.len().to_string()
    }

    fn part2(&self) -> String {
        let matrix: Vec<Vec<char>> = self.input.lines().map(|x| x.chars().collect()).collect();
        let height = matrix.len() as i128;
        let width = matrix[0].len() as i128;

        let mut antinode_locations: HashSet<Point> = HashSet::new();

        let frequency_station_mapper = self.get_stations(matrix);

        for (_, station_coordinates) in frequency_station_mapper.iter() {
            for f in self.get_antinodes_with_repeats(station_coordinates.to_vec(), height, width) {
                antinode_locations.insert(f);
            }
        }

        antinode_locations.len().to_string()
    }
}
