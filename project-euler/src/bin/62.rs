use std::collections::HashMap;

fn main() {
    let permutation_count = 5;
    let mut cube_tracker: HashMap<Vec<u8>, Vec<u128>> = HashMap::new();

    let mut i = 10u128;
    loop {
        i += 1;

        let mut key = (i * i * i)
            .to_string()
            .chars()
            .map(|z| z.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>();
        key.sort();

        let cube_count = cube_tracker.entry(key.to_owned()).or_insert(vec![]);
        cube_count.push(i * i * i);

        if cube_count.len() == permutation_count {
            println!("{} {:?}", i, cube_tracker.get(&key).unwrap());
            return;
        }
    }
}
