fn main() {
    let mut num = 600851475143u64;
    let mut factors = Vec::new();
    let mut d = 2;
    while num > 1 {
        while num % d == 0 {
            factors.push(d);
            num /= d;
        }
        d += 1;
        if d * d > num {
            if num > 1 {
                factors.push(num)
            }
            break;
        }
    }
    println!("{:?}", factors);
}
