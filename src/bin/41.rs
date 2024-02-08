fn next_permutation<T: Ord>(array: &mut [T]) -> bool {
    // Find non-increasing suffix
    if array.is_empty() {
        return false;
    }
    let mut i: usize = array.len() - 1;
    while i > 0 && array[i - 1] >= array[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }

    // Find successor to pivot
    let mut j: usize = array.len() - 1;
    while array[j] <= array[i - 1] {
        j -= 1;
    }
    array.swap(i - 1, j);

    // Reverse suffix
    array[i..].reverse();
    true
}

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

fn main() {
    let mut arr = [1, 2, 3, 4, 5, 6, 7];
    let mut largest_prime_permutation = 0;

    while next_permutation(&mut arr) {
        let n = arr
            .map(|x| x.to_string())
            .join("")
            .parse::<u64>()
            .unwrap_or(0);

        if is_prime(n) {
            largest_prime_permutation = n;
        }
    }

    println!("{}", largest_prime_permutation);
}
