/**
 * https://en.wikipedia.org/wiki/Fibonacci_sequence#Magnitude
 * https://www.desmos.com/calculator/hfhucugdnl
 * Using the provided formula...
 * the index of the first term in the Fibonacci sequence to contain 1000 digits was either 4781 or 4782 :)
 */

fn main() {
    let mut n = 1;
    loop {
        if (((1f64 + 5f64.sqrt()) / 2f64).log10() * (n as f64)).ceil() >= 1000f64 {
            println!("{n}");
            break;
        }
        n += 1;
    } 
}
