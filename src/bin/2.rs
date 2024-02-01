fn main() {
    let mut prev1 = 0;
    let mut prev2 = 1;
    let mut sum = 0;
    let mut tmp;

    while prev1 + prev2 < 4000000 {
        tmp = prev1;
        prev1 = prev1 + prev2;
        prev2 = tmp;

        if prev1 % 2 == 0 {
            sum += prev1;
            println!("{}", prev1);
        }
    }

    println!("{}", sum);
}
