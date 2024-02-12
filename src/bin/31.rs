fn main() {
    const COINS: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
    const AMOUNT: usize = 200;
    
    let mut ways = [0; AMOUNT + 1];
    ways[0] = 1;

    for coin in COINS {
        for i in coin..=AMOUNT {
            ways[i] += ways[i - coin];
        }
    }

    println!("Number of ways to make {}: {}", AMOUNT, ways[AMOUNT]);
}
