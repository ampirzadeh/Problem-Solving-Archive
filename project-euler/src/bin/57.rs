use num::{bigint::BigInt, FromPrimitive};

fn next_expansion(numerator: &BigInt, denominator: &BigInt) -> (BigInt, BigInt) {
    (numerator + 2 * denominator, numerator + denominator)
}
fn main() {
    let mut numerator = BigInt::from_u8(3).unwrap();
    let mut denominator = BigInt::from_u8(2).unwrap();
    let mut valid_expansion_cnt = 0;

    for _ in 0..1000 {
        let (n, d) = next_expansion(&numerator, &denominator);
        numerator = n;
        denominator = d;

        if amplib::order_of_magnitude_bigint(&numerator)
            > amplib::order_of_magnitude_bigint(&denominator)
        {
            valid_expansion_cnt += 1;
        }
    }
    println!("{:?}", valid_expansion_cnt);
}
