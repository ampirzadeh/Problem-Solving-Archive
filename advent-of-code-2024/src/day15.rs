use advent_of_code_2024::{Point, Solution};
pub struct Day15 {
    pub input: String,
}

enum Directions {
    Right,
    Left,
    Up,
    Down,
}

struct Warehouse {
    robot: Point,
    walls: Vec<Point>,
    boxes: Vec<Point>,
}

impl Warehouse {
    fn attempt_to_move(&mut self, dir: Directions) {
        let next = match dir {
            Directions::Right => Point::new(1, 0),
            Directions::Left => Point::new(-1, 0),
            Directions::Up => Point::new(0, -1),
            Directions::Down => Point::new(0, 1),
        };
        let mut to_check = self.robot + next;

        if self.walls.contains(&to_check) {
            return;
        }
        if self.boxes.contains(&to_check) == false && self.walls.contains(&to_check) == false {
            self.robot = self.robot + next;
            return;
        }

        let mut to_push: Vec<Point> = vec![];
        while self.boxes.contains(&to_check) {
            to_push.push(to_check);
            to_check = to_check + next;
        }

        if self.walls.contains(&to_check) == false {
            self.boxes = self
                .boxes
                .clone()
                .into_iter()
                .filter(|f| !to_push.contains(f))
                .collect();
            self.boxes.extend(to_push.iter().map(|f| *f + next));
            self.robot = self.robot + next;
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for i in 0..10 {
            for j in 0..10 {
                let p = Point::new(j, i);
                if self.walls.contains(&p) {
                    print!("#");
                } else if self.boxes.contains(&p) {
                    print!("O");
                } else if self.robot == p {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

impl Solution for Day15 {
    fn part1(&self) -> String {
        let mut warehouse = Warehouse {
            robot: Point::new(0, 0),
            boxes: vec![],
            walls: vec![],
        };

        let lines = self.input.lines();

        for (i, row) in lines.enumerate() {
            for (j, chr) in row.chars().enumerate() {
                match chr {
                    '#' => warehouse.walls.push(Point::new(j as i128, i as i128)),
                    '@' => warehouse.robot = Point::new(j as i128, i as i128),
                    'O' => warehouse.boxes.push(Point::new(j as i128, i as i128)),
                    '<' => warehouse.attempt_to_move(Directions::Left),
                    '>' => warehouse.attempt_to_move(Directions::Right),
                    '^' => warehouse.attempt_to_move(Directions::Up),
                    'v' => warehouse.attempt_to_move(Directions::Down),
                    _ => (),
                }
            }
        }

        let mut gps_score = 0;

        for b in warehouse.boxes {
            gps_score += (b.y) * 100 + (b.x);
        }

        gps_score.to_string()
    }

    fn part2(&self) -> String {
        // roses are red
        // violets are blue
        // i have not a single clue
        // of what i should do
        // lol bye
        todo!()
    }
}
