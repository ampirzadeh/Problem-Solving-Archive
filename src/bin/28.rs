/**
 * Top right corner of the square is (2n+1)^2
 * and every other corner rotating anti-clockwise is (2n+1)^2 - 2n, (2n+1)^2 - 4n, (2n+1)^2 - 6n
 * so the sum of all 4 corners is 4*(2n+1)^2 - 12n
 * we calculate the sum of all squares from n=1 to n=500 and add 1 to the sum for the center 1
 */
fn main() {
    let mut sum = 0;
    let n = 500;

    for i in 1..(n + 1) {
        sum += 4*(2*i + 1 as u64).pow(2) - 12*i;
    }
    sum += 1;
    println!("{sum}");
}