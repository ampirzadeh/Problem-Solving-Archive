/**
 * I originally wanted to implement a function that would calculate the last 10 digits of x^x with a u64;
 * You could just times those 10 digits by x, x times, and discard the carry, do this for all numbers upto 1000,
 * then calculate the sum using a similar method to question 13
 * 
 * But since I've already used BigInts for further questions, I just used it here too.
 */
use num::{BigInt, One, Zero};

fn main() {
    let n_last_digits = 10;
    let mut sum = BigInt::zero();
    for i in 1..1001 {
        let mut s = BigInt::one();
        for _ in 0..i {
            s *= i;
        }
        sum += s;
    }

    let mut s = sum.to_string();
    s.drain(0..sum.to_string().len() - n_last_digits);
    println!("{:?}", s);
}
