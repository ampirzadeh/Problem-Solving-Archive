fn main() {
    let mut sum = 0;
    for i in 1..1000001 {
        if amplib::is_palindrome(i.to_string()) && amplib::is_palindrome(format!("{i:b}")) {
            sum += i
        }
    }
    println!("{sum}");
}
