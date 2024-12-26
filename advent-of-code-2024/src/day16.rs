use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

use advent_of_code_2024::{Point, Solution};

pub struct Day16 {
    pub input: String,
}

struct Graph {
    vertices: Vec<Point>,
    start: Point,
    end: Point,
}

impl Solution for Day16 {
    fn part1(&self) -> i128 {
        let lines = self.input.lines();

        let mut g = Graph {
            vertices: vec![],
            start: Point::new(0, 0),
            end: Point::new(0, 0),
        };

        for (i, row) in lines.enumerate() {
            for (j, chr) in row.chars().enumerate() {
                let p = Point::new(j as i128, i as i128);
                match chr {
                    '#' => (),
                    '.' => g.vertices.push(p),
                    'E' => {
                        g.vertices.push(p);
                        g.end = p;
                    }
                    'S' => {
                        g.start = p;
                        g.vertices.push(p);
                    }
                    _ => (),
                }
            }
        }

        println!("start {:?}, end {:?}", g.start, g.end);

        #[derive(Eq, Hash, PartialEq)]
        enum Direction {
            North,
            East,
            West,
            South,
        }

        let mut dist: HashMap<Point, i128> = HashMap::new();
        let mut prev: HashMap<Point, Vec<Point>> = HashMap::new();
        let mut Q: VecDeque<Point> = VecDeque::new();

        for v in g.vertices {
            dist.insert(v, i128::MAX);
            //dist.insert(v, (Direction::North, i128::MAX));
            //dist.insert(v, (Direction::East, i128::MAX));
            //dist.insert(v, (Direction::West, i128::MAX));
            //dist.insert(v, (Direction::South, i128::MAX));
            prev.insert(v, vec![]);
            Q.push_back(v);
        }

        dist.insert(g.start, 0);

        while Q.is_empty() == false {
            let u = dist.iter();
        }
        // while Q is not empty:
        //u ← vertex in Q with minimum dist[u]
        //remove u from Q
        //
        //for each neighbor v of u still in Q:
        //alt ← dist[u] + Graph.Edges(u, v)
        //    if alt < dist[v]:
        //    dist[v] ← alt
        //prev[v] ← u
        //
        //    return dist[], prev[]

        0
    }

    fn part2(&self) -> i128 {
        // roses are red
        // violets are blue
        // i have not a single clue
        // of what i should do
        // lol bye
        0
    }
}
