// I LOVE RUST PATTERN MATCHING

use std::collections::HashMap;

use advent_of_code_2025::{Solution, Vector3d};
use itertools::Itertools;

pub struct Day8 {
    pub input: String,
}

impl Day8 {}

impl Solution for Day8 {
    fn part1(&self) -> String {
        const PAIRS_TO_TAKE_COUNT: usize = 1000;

        let mut points: Vec<Vector3d> = vec![];
        for line in self.input.lines() {
            let s = line
                .split(",")
                .map(|f| f.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            points.push(Vector3d {
                x: s[0],
                y: s[1],
                z: s[2],
            })
        }

        // This is basically a matrix of distances, except the matrix would have been symmetric and
        // so would have wasted memory
        // key: indices of source and destination in points[]
        // value: distance between the points
        let mut distance_map: HashMap<(usize, usize), i64> = HashMap::new();
        // build the distance map
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let dx = points[i].x - points[j].x;
                let dy = points[i].y - points[j].y;
                let dz = points[i].z - points[j].z;
                let sum = dx.pow(2) + dy.pow(2) + dz.pow(2);
                distance_map.insert((i, j), sum);
            }
        }

        // an iterator of the first 1000 pairs after they're sorted by distance
        let wanted_pairs = distance_map
            .iter()
            .sorted_by_key(|x| x.1)
            .take(PAIRS_TO_TAKE_COUNT);

        let mut circuits: Vec<Vec<usize>> = vec![];

        // add p1 and p2 to circuits appropriately
        for ((p1, p2), _d) in wanted_pairs {
            // Find circuits containing p1 or p2
            let c1_index = circuits.iter().position(|c| c.contains(p1));
            let c2_index = circuits.iter().position(|c| c.contains(p2));

            // PATTERN MATCHING IS GREAT
            match (c1_index, c2_index) {
                // neither is in any circuit so create a new circuit
                (None, None) => {
                    circuits.push(vec![*p1, *p2]);
                }

                // p1 is already in a circuit, add p2 to that circuit
                (Some(i), None) => {
                    circuits[i].push(*p2);
                }

                // p2 is already in a circuit, add p1 to that circuit
                (None, Some(i)) => {
                    circuits[i].push(*p1);
                }

                // each is in different circuits so merge two circuits
                (Some(i1), Some(i2)) if i1 != i2 => {
                    // Ensure i1 < i2 to avoid swapping issues
                    let (low, high) = if i1 < i2 { (i1, i2) } else { (i2, i1) };

                    let mut circuit2 = circuits.remove(high); // to avoid duplicated points
                    circuits[low].append(&mut circuit2);
                }

                // Only remaining scenario is when both are in the same circuit
                _ => {}
            }
        }

        circuits
            .iter()
            .map(|f| f.len()) // get the length of each circuit
            .sorted() // sort them ascending
            .rev() // sort them descending
            .take(3) // take the biggest three
            .product::<usize>() // multiply them
            .to_string() // and voila
    }

    fn part2(&self) -> String {
        0.to_string()
    }
}
