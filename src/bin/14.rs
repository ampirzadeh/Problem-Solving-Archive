fn collatz(n: u64) -> u64 {
    let mut num = n;
    let mut chain_cnt = 1;
    while num != 1 {
        chain_cnt += 1;
        if num % 2 == 0 {
            num = num / 2
        } else {
            num = 3 * num + 1;
        }
    }
    chain_cnt
}

fn main() {
    let mut max_chain = 1;
    let mut max_n = 1;
    
    for i in (1..1_000_000).rev() {
        let c = collatz(i);
        if c > max_chain {
            max_n = i;
            max_chain = c;
        }
    }
    println!("{max_n}: {max_chain}");
}
