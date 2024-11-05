// Using the square from question 28
// The corners: (2n+1)^2, (2n+1)^2 - 2n, (2n+1)^2 - 4n, (2n+1)^2 - 6n
// The first one is a perfect square so it's definitely not a prime
fn get_corners(n: u64) -> [u64; 4] {
    if n == 0 {
        return [1; 4];
    }
    let mut corners = [0; 4];
    corners[0] = (2 * n + 1) * (2 * n + 1);
    corners[1] = corners[0] - 2 * n;
    corners[2] = corners[0] - 4 * n;
    corners[3] = corners[0] - 6 * n;

    corners[0] = 1; // it's definitely not a prime so just make it 1 for performance

    corners
}
fn main() {
    let mut n = 1f64;
    let mut prime_count = 0f64;
    loop {
        for corner in get_corners(n as u64) {
            if amplib::is_prime(corner) {
                prime_count += 1f64;
            }
        }

        if (prime_count / ((n * 4f64) + 1f64)) < 0.1 {
            break;
        }

        n += 1f64;
    }
    println!("{}", n * 2f64 + 1f64);
}
