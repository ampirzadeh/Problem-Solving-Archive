fn main() {
    let mut product_digits = vec![1];
    let mut carry: u64 = 0;
    let factorial = 100;

    for i in 2..(factorial + 1) {
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
    println!("{:?}", product_digits.iter().sum::<u64>());
}
