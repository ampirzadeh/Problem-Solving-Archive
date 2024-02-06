fn is_palindrome(txt: String) -> bool {
    txt.chars().rev().collect::<String>() == txt
}

fn main() {
    let mut sum = 0;
    for i in 1..1000001 {
        if is_palindrome(i.to_string()) && is_palindrome(format!("{i:b}")) {
            sum += i
        }
    }
    println!("{sum}");
}
