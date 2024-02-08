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

fn goldbach_conj(n: u64) -> bool {
    if n % 2 == 0 {
        return false;
    }
    for i in 1..((n as f64).sqrt().ceil() as u64) {
        if n < i * i * 2 {
            return false;
        }
        if is_prime(n - i * i * 2) {
            return true;
        }
    }
    return false;
}

fn main() {
    let mut i = 3;
    loop {
        i += 2;
        if is_prime(i) == true {
            continue;
        }
        if goldbach_conj(i) == false {
            println!("{i}");
            return;
        }
    }
}
