fn main() {
    let mut amicable_sum = 0;
    let max = 10000;

    for i in 1..(max + 1) {
        let divs = amplib::factorisation(i);
        let sum = divs.iter().sum::<u64>() - i;
        if sum < max && sum > i {
            if amplib::factorisation(sum).iter().sum::<u64>() - sum == i {
                amicable_sum += i;
                amicable_sum += sum;
            }
        }
    }

    println!("{:?}", amicable_sum);
}
