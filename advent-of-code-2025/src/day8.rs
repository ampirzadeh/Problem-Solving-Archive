// I LOVE RUST PATTERN MATCHING
//
// My initial solution for part 1 was actually faster than my final solution but it required
// manually changing the number of pairs to take for part 2
// I tried having an increasing for loop for the number of pairs to take but it was too slow as it
// was calculating everything before it every time a new pair was being added
// So I decided to use an iterator instead to only calculate the insertion of the new pair. This
// removed the need for manual intervention but it meant that I couldn't get the n smallest pairs
// at once using select_nth_unstable_by_key, and had to do it one by one which affected performance
//
// Update: I thought of using a Binary Heap to get the smallest pairs 10 minutes after writing
// about the decreased performance so I gave it a go (instead of a HashMap) and it fixed all the
// performance issues! 0.81s for both parts :)

use std::collections::BTreeMap;

use advent_of_code_2025::{Solution, Vector3d};
use itertools::Itertools;

pub struct Day8 {
    pub input: String,
}

type Circuits = Vec<Vec<usize>>;
type DistanceMap = BTreeMap<i64, (usize, usize)>;

struct CircuitTracker {
    pub distance_map: DistanceMap,
    pub circuits: Circuits,
    pub last_pair: Option<(usize, usize)>,
}

impl Iterator for CircuitTracker {
    type Item = Circuits;

    fn next(&mut self) -> Option<Self::Item> {
        let Some((_, (p1, p2))) = self.distance_map.pop_first() else {
            return None;
        };

        self.last_pair = Some((p1, p2));

        // add p1 and p2 to circuits appropriately
        // Find circuits containing p1 or p2
        let c1_index = self.circuits.iter().position(|c| c.contains(&p1));
        let c2_index = self.circuits.iter().position(|c| c.contains(&p2));

        // PATTERN MATCHING IS GREAT
        match (c1_index, c2_index) {
            // neither is in any circuit so create a new circuit
            (None, None) => {
                self.circuits.push(vec![p1, p2]);
            }

            // p1 is already in a circuit, add p2 to that circuit
            (Some(i), None) => {
                self.circuits[i].push(p2);
            }

            // p2 is already in a circuit, add p1 to that circuit
            (None, Some(i)) => {
                self.circuits[i].push(p1);
            }

            // each is in different circuits so merge two circuits
            (Some(i1), Some(i2)) if i1 != i2 => {
                // Ensure i1 < i2 to avoid swapping issues
                let (low, high) = if i1 < i2 { (i1, i2) } else { (i2, i1) };

                let mut circuit2 = self.circuits.remove(high); // to avoid duplicated points
                self.circuits[low].append(&mut circuit2);
            }

            // Only remaining scenario is when both are in the same circuit
            _ => {}
        }

        Some(self.circuits.clone())
    }
}

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
        // I used a HashMap at first but it was too slow
        //
        // key: distance between the points
        // value: indices of source and destination in points[]
        let mut distance_map: DistanceMap = BTreeMap::new();
        // build the distance map
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let dx = points[i].x - points[j].x;
                let dy = points[i].y - points[j].y;
                let dz = points[i].z - points[j].z;
                let sum = dx.pow(2) + dy.pow(2) + dz.pow(2);
                distance_map.insert(sum, (i, j));
            }
        }

        distance_map
    }
}

impl Solution for Day8 {
    fn part1(&self) -> String {
        const PAIRS_TO_TAKE_COUNT: usize = 1000;

        let points = Self::get_points(&self.input);
        let distance_map = Self::get_distance_map(&points);

        let mut circuit_tracker = CircuitTracker {
            distance_map,
            circuits: vec![],
            last_pair: None,
        };

        let circuits = circuit_tracker.nth(PAIRS_TO_TAKE_COUNT - 1).unwrap();

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
        let points = Self::get_points(&self.input);
        let distance_map = Self::get_distance_map(&points);

        let mut circuit_tracker = CircuitTracker {
            distance_map,
            circuits: vec![],
            last_pair: None,
        };

        while let Some(circuits) = circuit_tracker.next() {
            // if the first circuit contains all the points
            if circuits[0].len() == points.len() {
                let last = circuit_tracker.last_pair.unwrap();
                return (points[last.0].x * points[last.1].x).to_string();
            }
        }

        0.to_string()
    }
}
