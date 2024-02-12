fn main() {
    let mut n = 1;
    let until = 20;

    for i in 2..(until + 1) {
        n = amplib::lcm(n, i);
    }

    println!("{n}");
}
