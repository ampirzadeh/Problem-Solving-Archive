fn next_circulation(n: u64) -> u64 {
    let mut new_n = n / 10;
    new_n += (n % 10) * 10_u64.pow(n.checked_ilog10().unwrap_or(0));
    new_n
}

fn main() {
    let mut flag;
    let mut num;
    let mut counter = 1; // 2

    for i in (3..1_000_001).step_by(2) {
        flag = true;
        num = i;
        loop {
            if !amplib::is_prime(num) {
                flag = false;
                break;
            }
            
            num = next_circulation(num);
            
            if num == i {
                break;
            }
        }
        
        if flag == true {
            counter += 1;
        }
    }
    println!("{}", counter);
}
