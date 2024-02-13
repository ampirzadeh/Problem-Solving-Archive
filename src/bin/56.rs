use num::{bigint::BigInt, One};

fn main() {
    let max = 100;
    let mut max_digit_sum = BigInt::from(0);
    let mut power;

    for base in 1..(max + 1) {
        power = BigInt::one();

        for _ in 1..(max + 1) {
            max_digit_sum = max_digit_sum.max(
                power
                    .to_string()
                    .chars()
                    .map(|x| x.to_digit(10).unwrap())
                    .sum(),
            );
            power *= base;
        }
    }

    println!("{}", max_digit_sum);
}
