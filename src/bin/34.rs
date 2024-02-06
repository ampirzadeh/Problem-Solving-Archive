fn main() {
    let mut factorials = [1; 10];
    for i in 1..10 {
        factorials[i] = factorials[i - 1] * i;
    }
    let mut i = 10;
    while i <= 9999999  {
        let mut fact_sum = 0;
        let mut j = i;
        while j > 0 {
            fact_sum += factorials[j % 10];
            j /= 10;
        }
        if fact_sum == i {
            println!("{fact_sum}");
        }
        i += 1;
    }
}
