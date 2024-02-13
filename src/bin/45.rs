fn main() {
    let mut n: u64 = 2;
    loop {
        if amplib::is_pentagonal(n) && amplib::is_hexagonal(n) && amplib::is_triangle(n) {
            println!("{}", n);
        }
        n += 1;
    }
}
