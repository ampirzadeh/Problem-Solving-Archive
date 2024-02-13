fn reverse_number(n: u128) -> u128 {
    let mut reversed = 0;
    let mut n = n;
    while n > 0 {
        reversed *= 10;
        reversed += n % 10;
        n /= 10;
    }
    reversed
}

fn next_iteration(n: u128) -> u128 {
    n + reverse_number(n)
}

fn main() {
    let mut lychrel_counter = 0;
    let mut n = 1;
    let mut iteration_counter = 0;
    let mut num = n;
    while n <= 10_000 {
        num = next_iteration(num);
        iteration_counter += 1;

        if iteration_counter > 50 {
            lychrel_counter += 1;
            iteration_counter = 0;
            n += 1;
            num = n;
            continue;
        }
        if amplib::is_palindrome(num.to_string()) {
            iteration_counter = 0;
            n += 1;
            num = n;
            continue;
        }
    }
    
    println!("{lychrel_counter}");
}
