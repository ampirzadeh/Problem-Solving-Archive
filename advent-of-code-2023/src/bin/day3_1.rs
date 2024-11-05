use std::{
    fs::File,                 // file system
    io::{BufRead, BufReader}, // input/output
    path::Path,               // path
};

fn main() {
    let p = Path::new("src/data/day3.txt");
    let file = File::open(&p).unwrap();
    let reader = BufReader::new(&file);
    let mut lines = reader.lines();
    let mut sum: u32 = 0;

    let prev = lines.next().unwrap().unwrap();
    let mut prev = prev.chars();
    let main = lines.next().unwrap().unwrap();
    let mut main = main.chars();
    let next = lines.next().unwrap().unwrap();
    let mut next = next.chars();

    // println!("{:?}\n{:?}\n{:?}", prev, main, next.nth(94).unwrap_or('.') == '*');

    let mut flag = false;
    let mut n = 0u32;

    for (idx, chr) in main.enumerate() {
        if chr.is_digit(10) {
            n = n * 10 + chr.to_digit(10).unwrap();

            // up
            if prev.nth(idx).unwrap_or('.') == '*' {
                flag = true;
            }
            // down
            if next.nth(idx).unwrap_or('.') == '*' {
                flag = true;
            }
            // up left
            if prev.nth(idx - 1).unwrap_or('.') == '*' {
                flag = true;
            }
            // up right
            if prev.nth(idx + 1).unwrap_or('.') == '*' {
                flag = true;
            }
            // down left
            if next.nth(idx - 1).unwrap_or('.') == '*' {
                flag = true;
            }
            // down right
            if next.nth(idx + 1).unwrap_or('.') == '*' {
                flag = true;
            }

            print!("{:?}", &next);
        } else {
            if flag == true {
                println!("{n}");
                sum += n;
            } else if chr == '*' {
                flag = true;

                if n != 0 {
                    println!("{n}");
                }
                n = 0;
            }
            if chr != '*' {
                n = 0;

                flag = false;
            }
        }
    }
}
