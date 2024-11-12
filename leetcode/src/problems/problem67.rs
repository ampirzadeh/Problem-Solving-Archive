pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a_char_rev = a.chars().rev();
        let mut b_chars_rev = b.chars().rev();
        let mut res = String::new();
        let mut carry = 0;

        loop {
            let a_digit = a_char_rev.nth(0);
            let b_digit = b_chars_rev.nth(0);
            if a_digit.is_none() && b_digit.is_none() && carry == 0 {
                println!("{}", res);
                break;
            }

            let sum = carry
                + a_digit
                    .unwrap_or('0')
                    .to_digit(10)
                    .expect("Invalid binary string a")
                + b_digit
                    .unwrap_or('0')
                    .to_digit(10)
                    .expect("Invalid binary string b");
            res.insert(0, char::from_digit(sum % 2, 10).expect("Invalid binary"));
            carry = sum / 2;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::add_binary("0".to_string(), "0".to_string()),
            "0".to_string()
        );
    }
}
