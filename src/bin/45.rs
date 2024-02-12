// T(n)=(n^2+n)/2=N => n^2+n-2N=0 => n=1/2 (sqrt(8N+1)-1)
// P(n)=(3n^2+n)/2=N => 3n^2-n-2N=0 => n=1/6 (sqrt(24N+1)+1)
// H(n)=(2n^2-n)=N => 2n^2-n-N=0 => n=1/4 (sqrt(8N+1)+1)

fn main() {
    let mut n: u64 = 2;
    loop {
        if amplib::is_pentagonal(n) && amplib::is_hexagonal(n) && amplib::is_triangle(n) {
            println!("{}", n);
        }
        n += 1;
    }
}
