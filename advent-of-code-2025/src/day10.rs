use advent_of_code_2025::Solution;
use pathfinding::prelude::bfs;
use regex::Regex;

pub struct Day10 {
    pub input: String,
}

type MachineState = Vec<bool>; // true for on and false for off
type Button = Vec<usize>; // the indices that the button will toggle
type JoltageState = Vec<usize>; // the current joltages

impl Day10 {
    fn get_machine_info(line: &str) -> (MachineState, Vec<Button>, JoltageState) {
        // I love regex but this is not readable for other developers, unless through tears
        //
        // As the saying goes: When I first wrote this code only God and I knew how it worked
        // A month later, only God knows
        //
        // "target_state": ".##.", "buttons": "(3) (1,3) (2) (2,3) (0,2) (0,1)", "joltages": "3,5,4,7"
        let re = Regex::new(r"\[(?<target_state>.*)\]\s+(?<buttons>\(.*\))\s+\{(?<joltages>.*)\}")
            .unwrap();

        let cap = re.captures(line).unwrap();

        let target_state_str = cap.name("target_state").unwrap().as_str();
        let buttons_str = cap.name("buttons").unwrap().as_str();
        let joltages_str = cap.name("joltages").unwrap().as_str();

        let target_state = target_state_str
            .chars()
            .map(|c| c == '#')
            .collect::<MachineState>();

        let buttons = buttons_str
            .split(" ")
            .map(|button_str| {
                let button_str = button_str.trim_matches(|c| c == '(' || c == ')');

                button_str
                    .split(",")
                    .map(|t| t.parse::<usize>().unwrap())
                    .collect()
            })
            .collect::<Vec<Button>>();

        let target_joltages = joltages_str
            .split(",")
            .map(|t| t.parse::<usize>().unwrap())
            .collect::<JoltageState>();

        return (target_state, buttons, target_joltages);
    }

    fn possible_toggle_states(
        current_state: &MachineState,
        buttons: &Vec<Button>,
    ) -> Vec<MachineState> {
        let mut possible_states: Vec<MachineState> = vec![];

        for button in buttons {
            let mut new_state = current_state.clone();

            // toggle the indices indicated by the button
            for button_index in button {
                new_state[*button_index] = !new_state[*button_index];
            }
            possible_states.push(new_state);
        }
        possible_states
    }

    fn possible_joltage_states(
        current_joltage: &JoltageState,
        buttons: &Vec<Button>,
    ) -> Vec<JoltageState> {
        let mut possible_states: Vec<JoltageState> = vec![];

        for button in buttons {
            let mut new_state = current_joltage.clone();

            // increase the joltages indicated by the button
            for button_index in button {
                new_state[*button_index] += 1;
            }
            possible_states.push(new_state);
        }
        possible_states
    }
}

impl Solution for Day10 {
    fn part1(&self) -> String {
        let mut min_button_press_sum = 0;

        for line in self.input.lines() {
            let (target_state, buttons, _) = Self::get_machine_info(line);

            let initial_state = vec![false; target_state.len()]; // everything is off at first

            // slightttttly cheating to use a library function but in the real-world one would do it.
            let result = bfs(
                &initial_state,
                |p| Self::possible_toggle_states(p, &buttons),
                |p| *p == target_state,
            );

            min_button_press_sum += result.unwrap().len() - 1; // minus 1 because the initial state is included but shouldn't be
        }

        min_button_press_sum.to_string()
    }

    fn part2(&self) -> String {
        // CRASHES THE COMPUTER
        // GAVE UP FOR NOW
        let mut min_button_press_sum = 0;

        let mut i = 0;
        for line in self.input.lines() {
            i += 1;
            println!("{line} {i}");

            let (_, buttons, target_joltages) = Self::get_machine_info(line);

            let initial_state = vec![0; target_joltages.len()]; // 0 joltage at first

            let result = bfs(
                &initial_state,
                |p| {
                    // check if the current state is already invalid
                    for i in 0..p.len() {
                        if p[i] > target_joltages[i] {
                            return vec![];
                        }
                    }
                    Self::possible_joltage_states(p, &buttons)
                },
                |p| *p == target_joltages,
            );

            min_button_press_sum += result.unwrap().len() - 1; // minus 1 because the initial state is included but shouldn't be
        }

        min_button_press_sum.to_string()
    }
}
