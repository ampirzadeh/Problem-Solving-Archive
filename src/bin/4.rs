use std::cmp;

fn is_palindrome(n: u64) -> bool {
    let mut num = n;
    let mut rev = 0;
    while num > 0 {
        let dig = num % 10;
        rev = rev * 10 + dig;
        num = num / 10;
    }

    n == rev
}

fn main() {
    let mut m = 0;

    for i in (1..999).rev() {
        for j in (1..999).rev() {
            if is_palindrome(i * j) {
                m = cmp::max(m, i * j);
            }
        }
    }
    println!("{}", m);
}
