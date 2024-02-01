fn check_prime(n: usize) -> bool {
    // Iterate from i = 2 to sqrt ( n )
    let mut i: usize = 2;
    while i * i <= n {
        // Return false if the number is divisible
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    // Return true finally
    return true;
}

fn main() {
    let mut sum = 2;
    for i in (3..2_000_000).step_by(2) {
        if check_prime(i) {
            sum += i;
        }
    }
    println!("{sum}");
}
