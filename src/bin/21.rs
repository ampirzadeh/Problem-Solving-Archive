// Same function used in question 12
fn find_divisors(n: i32) -> Vec<i32> {
    let mut divs = Vec::new();
    for i in 1..((n as f64).sqrt() + 1f64) as i32 {
        if n % i == 0 {
            divs.push(i);
            if i != n / i {
                divs.push(n / i);
            }
        }
    }
    divs
}

fn main() {
    let mut amicable_sum = 0;
    let max = 10000;

    for i in 1..(max + 1) {
        let divs = find_divisors(i);
        let sum = divs.iter().sum::<i32>() - i;
        if sum < max && sum > i {
            if find_divisors(sum).iter().sum::<i32>() - sum == i {
                amicable_sum += i;
                amicable_sum += sum;
            }
        }
    }

    println!("{:?}", amicable_sum);
}
