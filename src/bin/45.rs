// T(n)=(n^2+n)/2=N => n^2+n-2N=0 => n=1/2 (sqrt(8N+1)-1)
// P(n)=(3n^2+n)/2=N => 3n^2-n-2N=0 => n=1/6 (sqrt(24N+1)+1)
// H(n)=(2n^2-n)=N => 2n^2-n-N=0 => n=1/4 (sqrt(8N+1)+1)

fn is_triangle(x: u128) -> bool {
    let n = (((8 * x + 1) as f64).sqrt() - 1f64) / 2f64;
    return n == n.trunc();
}
fn is_pentagonal(x: u128) -> bool {
    let n = (((24 * x + 1) as f64).sqrt() + 1f64) / 6f64;
    return n == n.trunc();
}
fn is_hexagonal(x: u128) -> bool {
    let n = (((8 * x + 1) as f64).sqrt() + 1f64) / 4f64;
    return n == n.trunc();
}

fn main() {
    let mut n: u128 = 2;
    loop {
        if is_pentagonal(n) && is_hexagonal(n) && is_triangle(n) {
            println!("{}", n);
        }
        n += 1;
    }
}
