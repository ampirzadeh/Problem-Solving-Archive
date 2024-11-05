fn goldbach_conj(n: u64) -> bool {
    if n % 2 == 0 {
        return false;
    }
    for i in 1..((n as f64).sqrt().ceil() as u64) {
        if n < i * i * 2 {
            return false;
        }
        if amplib::is_prime(n - i * i * 2) {
            return true;
        }
    }
    false
}

fn main() {
    let mut i = 3;
    loop {
        i += 2;
        if amplib::is_prime(i) == true {
            continue;
        }
        if goldbach_conj(i) == false {
            break;
        }
    }
    println!("{i}");
}
