fn main() {
    for b in 1..500 {
        if 1000 * (500 - b) % (1000 - b) == 0 {
            println!(
                "{}*{}*{}={}",
                b,
                1000 * (500 - b) / (1000 - b),
                1000 - b - 1000 * (500 - b) / (1000 - b),
                b * (1000 * (500 - b) / (1000 - b)) * (1000 - b - 1000 * (500 - b) / (1000 - b))
            );
        }
    }
}
