use std::collections::HashMap;

use advent_of_code_2025::Solution;

pub struct Day11 {
    pub input: String,
}

type Node = String; // just for clarity
type AdjacencyList = HashMap<Node, Vec<Node>>;

// Recursively counts the number of possible routes from `current_node` to `end`
// `memo` is a cache of previously calculated route counts and the key to the performance
fn count_paths(
    graph: &AdjacencyList,
    current_node: &Node,
    end: &Node,
    memo: &mut HashMap<Node, u128>,
) -> u128 {
    // base case
    if current_node == end {
        return 1;
    }

    // if we've already calculated it before
    if let Some(&cached) = memo.get(current_node) {
        return cached;
    }

    // add up all the possible routes that can be takes from each of the neighbours
    let mut total = 0;
    if let Some(neighbours) = graph.get(current_node) {
        for next in neighbours {
            total += count_paths(&graph, next, end, memo);
        }
    }

    // cache the calculated value
    memo.insert(current_node.clone(), total);
    total
}

impl Day11 {
    fn create_adjacency(input: &String) -> AdjacencyList {
        let mut adjacency_list: AdjacencyList = HashMap::new();

        for line in input.lines() {
            let (device, outputs) = line.split_once(":").unwrap();
            let output_devices = outputs
                .trim()
                .split_whitespace()
                .map(|f| f.to_string())
                .collect::<Vec<Node>>();
            adjacency_list.insert(device.to_string(), output_devices);
        }

        adjacency_list
    }
}

impl Solution for Day11 {
    fn part1(&self) -> String {
        let adjacency_list = Day11::create_adjacency(&self.input);

        let from: Node = "you".to_string();
        let to: Node = "out".to_string();
        let n_of_ways = count_paths(&adjacency_list, &from, &to, &mut HashMap::new());

        n_of_ways.to_string()
    }

    fn part2(&self) -> String {
        let adjacency_list = Day11::create_adjacency(&self.input);

        let from: Node = "svr".to_string();
        let to: Node = "out".to_string();

        let mut n_of_ways = 0u128;

        // the number of ways where: svr -> fft -> dac -> out
        {
            let dest_1 = "fft".to_string();
            let dest_2 = "dac".to_string();
            n_of_ways += count_paths(&adjacency_list, &from, &dest_1, &mut HashMap::new())
                * count_paths(&adjacency_list, &dest_1, &dest_2, &mut HashMap::new())
                * count_paths(&adjacency_list, &dest_2, &to, &mut HashMap::new());
        }

        // the number of ways where: svr -> dac -> fft -> out
        {
            let dest_1 = "dac".to_string();
            let dest_2 = "fft".to_string();
            n_of_ways += count_paths(&adjacency_list, &from, &dest_1, &mut HashMap::new())
                * count_paths(&adjacency_list, &dest_1, &dest_2, &mut HashMap::new())
                * count_paths(&adjacency_list, &dest_2, &to, &mut HashMap::new());
        }

        n_of_ways.to_string()
    }
}
