use std::fs;

fn is_triangle(x: u64) -> bool {
    let n = ((8 * x + 1) as f64).sqrt();
    n == n.trunc()
}

fn main() {
    // ! Provide file
    let file_contents = fs::read_to_string("./src/bin/words.txt").unwrap();

    let words = file_contents.replace(r#"""#, "");
    let words: Vec<&str> = words.split(",").collect();
    let mut sum;
    let mut cnt = 0;
    for word in words {
        sum = 0;
        for letter in word.chars() {
            sum += letter as u64 - 64;
        }
        if is_triangle(sum) {
            cnt += 1;
        }
    }
    println!("{cnt}");
}
