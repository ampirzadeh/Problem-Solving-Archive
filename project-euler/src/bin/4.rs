use std::cmp;

fn main() {
    let mut m = 0;

    for i in (1..999).rev() {
        for j in (1..999).rev() {
            if amplib::is_palindrome((i * j).to_string()) {
                m = cmp::max(m, i * j);
            }
        }
    }

    println!("{}", m);
}
