fn main() {
    let mut consecutive_count = 0;
    let mut i = 2 * 3 * 5 * 7;

    loop {
        i += 1;

        if amplib::prime_factors(i).len() == 4 {
            consecutive_count += 1;
        } else {
            consecutive_count = 0;
        }

        if consecutive_count == 4 {
            println!("{}", i - 3);
            break;
        }
    }
}
