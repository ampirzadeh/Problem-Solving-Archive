fn main() {
    let a = String::from("*");
    println!("{}", a.chars().nth(0).unwrap().is_digit(10));
}
