fn main() {
    let nth_prime = 10_001;

    let mut found_primes: Vec<u64> = Vec::new();
    found_primes.push(2);

    let mut to_check_if_prime = 3;
    let mut is_prime;

    while found_primes.len() < nth_prime {
        is_prime = true;

        // if the number is divisible by a previously found prime, it's not a prime
        for prime in &found_primes {
            if to_check_if_prime % prime == 0 {
                is_prime = false;
            }
        }

        if is_prime == true {
            found_primes.push(to_check_if_prime);
        }

        to_check_if_prime += 2;
    }

    println!("{}", found_primes.get(found_primes.len() - 1).unwrap())
}
