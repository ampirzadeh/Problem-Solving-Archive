fn main() {
    let mut arr = [1, 2, 3, 4, 5, 6, 7];
    let mut largest_pandigital_prime = 0;

    while amplib::next_permutation(&mut arr) {
        let n = arr
            .map(|x| x.to_string())
            .join("")
            .parse::<u64>()
            .unwrap_or(0);

        if amplib::is_prime(n) {
            largest_pandigital_prime = n;
        }
    }

    println!("{}", largest_pandigital_prime);
}
