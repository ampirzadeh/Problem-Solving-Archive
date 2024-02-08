fn factor_count(n: u64) -> usize {
    let mut num = n;
    let mut factors = Vec::new();
    let mut d = 2;
    while num > 1 {
        if num % d == 0 {
            factors.push(d);
            while num % d == 0 {
                num /= d;
            }
        }
        d += 1;
        if d * d > num {
            if num > 1 {
                factors.push(num)
            }
            break;
        }
    }

    return factors.len();
}

fn main() {
    let mut consecutive_count = 0;
    let mut i = 2 * 3 * 5 * 7;

    loop {
        i += 1;

        if factor_count(i) == 4 {
            consecutive_count += 1;
        } else {
            consecutive_count = 0;
        }

        if consecutive_count == 4 {
            println!("{}", i - 3);
            break;
        }
    }
}
