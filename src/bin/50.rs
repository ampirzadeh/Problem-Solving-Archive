// !
fn main() {
    let until = 1_000_000;

    let mut found_primes: Vec<u64> = Vec::new();
    found_primes.push(2);

    let mut to_check_if_prime = 1;

    'outer: while found_primes[&found_primes.len() - 1] < until / 100 {
        to_check_if_prime += 2;
        for prime in &found_primes {
            if to_check_if_prime % prime == 0 {
                continue 'outer;
            }
        }

        found_primes.push(to_check_if_prime);
    }

    let mut prime_sum: Vec<u64> = Vec::new();
    for p in found_primes {
        prime_sum.push(prime_sum.last().unwrap_or(&0) + p);
        if prime_sum[prime_sum.len() - 1] >= until {
            break;
        }
    }

    let mut max_prime = 0;
    let mut terms = 0;

    for i in 0..prime_sum.len() {
        for j in ((i + terms)..(prime_sum.len() - 1)).rev() {
            let n = prime_sum[j] - prime_sum[i];
            if j - i > terms && amplib::is_prime(n) {
                terms = j - i;
                max_prime = n;
                break;
            }
        }
    }

    println!("chain_sum {max_prime} chain_length {terms}");
}
