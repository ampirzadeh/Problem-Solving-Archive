// I LOVE RUST PATTERN MATCHING

use std::collections::HashMap;

use advent_of_code_2025::{Solution, Vector3d};
use itertools::Itertools;

pub struct Day8 {
    pub input: String,
}

type Circuits = Vec<Vec<usize>>;
type DistanceMap = HashMap<(usize, usize), i64>;

impl Day8 {
    fn get_points(input: &String) -> Vec<Vector3d> {
        let mut points: Vec<Vector3d> = vec![];
        for line in input.lines() {
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
        points
    }

    fn get_distance_map(points: &Vec<Vector3d>) -> DistanceMap {
        // This is basically a matrix of distances, except the matrix would have been symmetric and
        // so would have wasted memory
        // key: indices of source and destination in points[]
        // value: distance between the points
        let mut distance_map: DistanceMap = HashMap::new();
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

        distance_map
    }
    fn get_circuits(
        distance_map: &DistanceMap,
        pairs_to_take_count: usize,
        points: &Vec<Vector3d>,
    ) -> Circuits {
        // wanted_pairs is an iterator of the n smallest pairs after they're sorted by distance
        //
        // My initial Solution:
        // let wanted_pairs = distance_map
        //     .iter()
        //     .sorted_by_key(|x| x.1)
        //     .take(pairs_to_take_count);
        //
        // Much better performance as it doesn't sort the whole distance_map
        let mut items: Vec<_> = distance_map.iter().collect();
        if items.len() > pairs_to_take_count {
            items.select_nth_unstable_by_key(pairs_to_take_count, |x| x.1);
        }
        let wanted_pairs = items[..pairs_to_take_count].iter();

        let mut circuits: Circuits = vec![];

        // add p1 and p2 to circuits appropriately
        for ((p1, p2), _d) in wanted_pairs {
            // Find circuits containing p1 or p2
            let c1_index = circuits.iter().position(|c| c.contains(p1));
            let c2_index = circuits.iter().position(|c| c.contains(p2));

            // println!(
            //     "adding {p1} and {p2} which are points in circuits {:?} and {:?}",
            //     points[*p1], points[*p2]
            // );
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
    }
}

impl Solution for Day8 {
    fn part1(&self) -> String {
        const PAIRS_TO_TAKE_COUNT: usize = 10;

        let points = Self::get_points(&self.input);
        let distance_map = Self::get_distance_map(&points);
        let circuits = Self::get_circuits(&distance_map, PAIRS_TO_TAKE_COUNT, &points);

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
        let mut pair_count = 3580;

        let points = Self::get_points(&self.input);
        let distance_map = Self::get_distance_map(&points);

        let mut circuits;
        loop {
            circuits = Self::get_circuits(&distance_map, pair_count, &points);

            let a = distance_map
                .iter()
                .sorted_by_key(|x| x.1)
                .nth(pair_count - 1)
                .unwrap()
                .to_owned();
            println!(
                "ADDING {} AND {} WHICH ARE POINTS IN CIRCUITS {:?} AND {:?} -> {}",
                a.0 .0, a.0 .1, points[a.0 .0], points[a.0 .1], points[a.0 .0].x * points[a.0 .1].x
            );

            pair_count += 1;
            println!("{} {}", pair_count, circuits.len());

            if circuits.len() == 1 {
                break;
            }
        }

        0.to_string()
    }
}
