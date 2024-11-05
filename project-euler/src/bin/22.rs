use std::fs;

fn get_score(name: &str) -> usize {
    let mut score = 0;
    for c in name.chars() {
        score += c as usize - 64;
    }
    score
}

fn main() {
    // ! Provide file
    let file_contents = fs::read_to_string("./src/bin/names.txt").unwrap();

    let all_names = file_contents.replace(r#"""#, "");
    let mut all_names: Vec<&str> = all_names.split(",").collect();
    all_names.sort();
    
    let mut score_sum = 0;
    for (i, name) in all_names.iter().enumerate() {
        score_sum += get_score(name) * (i + 1);
    }
    println!("{:?} {}", all_names, score_sum);
}
