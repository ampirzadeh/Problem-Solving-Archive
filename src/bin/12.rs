fn main() {
    let mut n = 1;

    while amplib::factorisation((n * (n+1)) / 2).len() < 500 {
        n += 1;
    }

    println!("{}", (n * (n+1)) / 2);
}