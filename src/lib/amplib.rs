pub fn factorisation(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    for i in 1..((n as f64).sqrt() + 1f64).ceil() as u64 {
        if n % i == 0 {
            factors.push(i);
            if i != n / i {
                factors.push(n / i);
            }
        }
    }
    factors
}

pub fn prime_factorisation(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut d = 2;
    let mut num = n;
    while num > 1 {
        while num % d == 0 {
            factors.push(d);
            num /= d;
        }
        d += 1;
        if d * d > num {
            if num > 1 {
                factors.push(num)
            }
            break;
        }
    }
    factors
}

pub fn prime_factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut d = 2;
    let mut num = n;
    while num > 1 {
        while num % d == 0 {
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
    factors
}

pub fn is_palindrome(txt: String) -> bool {
    txt.chars().rev().collect::<String>() == txt
}

pub fn lcm(n1: u64, n2: u64) -> u64 {
    let mut max = if n1 > n2 { n1 } else { n2 };

    loop {
        if max % n1 == 0 && max % n2 == 0 {
            return max;
        }
        max += 1;
    }
}

pub fn gcd(x: u32, y: u32) -> u32 {
    if y == 0 {
        return x;
    }
    gcd(y, x % y)
}

pub fn factorial(n: u64) -> Vec<u64> {
    let mut product_digits = vec![1];
    let mut carry = 0;

    for i in 2..(n + 1) {
        for digit in (0..product_digits.len()).rev() {
            let d = product_digits[digit];
            product_digits[digit] = (d * i + carry) % 10;
            carry = (d * i + carry) / 10;
        }
        while carry > 0 {
            product_digits.insert(0, carry % 10);
            carry /= 10;
        }
    }

    product_digits
}

pub fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n == 1 || n % 2 == 0 {
        return false;
    }
    for i in (3..((n as f64).sqrt().ceil() + 1f64) as u64).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

pub fn is_n_digit_pandigital(n: u64) -> bool {
    let n_str = n.to_string();
    for i in 1..n_str.len() {
        if !n_str.contains(&i.to_string()) {
            return false;
        }
    }

    true
}

pub fn is_triangle(x: u64) -> bool {
    let n = (((8 * x + 1) as f64).sqrt() - 1f64) / 2f64;
    n == n.trunc()
}

pub fn is_pentagonal(x: u64) -> bool {
    let n = (((24 * x + 1) as f64).sqrt() + 1f64) / 6f64;
    n == n.trunc()
}

pub fn is_hexagonal(x: u64) -> bool {
    let n = (((8 * x + 1) as f64).sqrt() + 1f64) / 4f64;
    n == n.trunc()
}

// https://www.nayuki.io/page/next-lexicographical-permutation-algorithm
pub fn next_permutation<T: Ord>(array: &mut [T]) -> bool {
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
