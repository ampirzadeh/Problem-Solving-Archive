fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n == 1 || n % 2 == 0 {
        return false;
    }
    for i in 3..(((n as f64).sqrt().ceil()) as u64) {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}
fn is_n_digit_pandigital(n: u64) -> bool {
    let n_str = n.to_string();
    for i in 1..n_str.len() {
        if !n_str.contains(&i.to_string()) {
            return false;
        }
    }

    return true;
}

fn main() {
    let mut i = 987654321;
    while i > 0 {
        println!("{i}");
        if is_n_digit_pandigital(i) && is_prime(i) {
            return;
        }
        i -= 2;
    }
}
