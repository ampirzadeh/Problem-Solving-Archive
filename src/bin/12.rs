fn find_divisors(n: i32) -> Vec<i32> {
    let mut divs = Vec::new();
    for i in 1..((n as f64).sqrt() + 1f64) as i32 {
        if n % i == 0 {
            divs.push(i);
            if i != n / i {
                divs.push(n / i);
            }
        }
    }
    divs
}

fn main() {
    let mut n = 1;

    while find_divisors((n * (n+1)) / 2).len() < 500 {
        n += 1;
    }

    println!("{}", (n * (n+1)) / 2);
}