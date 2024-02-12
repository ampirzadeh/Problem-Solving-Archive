// 9! = 362 880
// 9! * 2 = 725 760
// 1 000 000 - 9! * n > 0 => floor(n) = 2
/**
 * I originally solved this problem by hand using the above method,
 * but when I came across other permutation problems later on I decided to change my answer to a more comprehensive one.
*/

fn main() {
    let mut digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for _ in 1..1_000_000 {
        amplib::next_permutation(&mut digits);
    }
    println!("{:?}", digits.map(|x| x.to_string()).join(""));
}