const N_OF_DIGITS: u32 = 8;

fn trimmed_pow(base: u128, exp: u128) -> u128 {
    let mut res = 1;

    let mut exp = exp;
    let mut base = base;

    while exp > 0 {
        if exp & 1 == 1 {
            res = (res * base) % 10u128.pow(N_OF_DIGITS);
        }

        base = (base * base) % 10u128.pow(N_OF_DIGITS);
        exp >>= 1;
    }

    res
}

fn trimmed_tetrate(base: u128, exp: u128) -> u128 {
    let mut last = 0;
    let mut res = 1;

    for _ in 0..exp {
        res = trimmed_pow(base, res);

        // break early if converged
        if last == res {
            break;
        }
        last = res
    }

    res
}

fn main() {
    println!("{}", trimmed_tetrate(1777, 1855));
}
