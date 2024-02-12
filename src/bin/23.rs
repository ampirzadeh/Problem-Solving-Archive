// TODO significant room for performance improvements
fn main() {
    let mut abundant_numbers: Vec<u64> = Vec::new();
    for i in 1..28123 {
        if i > 1 && amplib::factorisation(i).iter().sum::<u64>() - i > i {
            abundant_numbers.push(i);
        }
    }

    let mut sum = 0;
    for i in 1..28123 {
        let mut flag = true;
        for ab in abundant_numbers.to_owned() {
            if i > ab && abundant_numbers.contains(&(i - ab)) {
                flag = false;
                break;
            }
        }
        if flag == true {
            sum += i;
        }
    }

    println!("{:?}", sum);
}