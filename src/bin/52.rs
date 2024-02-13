fn are_anagrams(n1: u128, n2: u128) -> bool {
    let n1_str = n1.to_string();
    let n2_str = n2.to_string();
    
    for n1_digit in n1_str.chars() {
        let mut flag = false;
        for n2_digit in n2_str.chars() {
            if n1_digit == n2_digit {
                flag = true;
            }
        }
        if flag == false {
            return false;
        }
    }
    
    for n2_digit in n2_str.chars() {
        let mut flag = false;
        for n1_digit in n1_str.chars() {
            if n1_digit == n2_digit {
                flag = true;
            }
        }
        if flag == false {
            return false;
        }
    }
    
    true
}

fn main() {
    let mut i = 2;
    loop {
        if are_anagrams(i, 2*i) && are_anagrams(i, 3*i) && are_anagrams(i, 4*i) && are_anagrams(i, 5*i) && are_anagrams(i, 6*i) {
            println!("{i}");
            break;
        }
        i += 1;
    }
}
