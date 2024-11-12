pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut x_rev = 0;
        let mut x_copy = x;
        while x_copy > 0 {
            x_rev *= 10;
            x_rev += x_copy % 10;
            x_copy /= 10;
        }
        x_rev == x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
