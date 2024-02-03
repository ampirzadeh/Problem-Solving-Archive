fn find_divisors(n: usize) -> Vec<usize> {
    let mut divs = Vec::new();
    for i in 1..((n as f64).sqrt() + 1f64) as usize {
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
    let mut abundant_numbers: Vec<usize> = Vec::new();
    for i in 1..28123 {
        if i > 1 && find_divisors(i).iter().sum::<usize>() - i > i {
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