fn is_rtl_truncatable(n: u64) -> bool {
    let mut num = n;
    while num > 0 {
        if !amplib::is_prime(num) {
            return false;
        }
        num /= 10;
    }
    true
}

fn is_ltr_truncatable(n: u64) -> bool {
    let mut num = n;
    while num > 0 {
        let mag = 10_u64.pow(num.checked_ilog10().unwrap_or(0));
        if !amplib::is_prime(num) {
            return false;
        }
        num -= (num / mag) * mag;
    }
    true
}

fn main() {
    let mut i = 3;
    let mut sum = 0;
    let mut counter = 1;

    while counter <= 11 {
        if i > 10 && is_rtl_truncatable(i) && is_ltr_truncatable(i) {
            sum += i;
            counter += 1;
        }
        i += 2;
    }

    println!("{sum}");
}
