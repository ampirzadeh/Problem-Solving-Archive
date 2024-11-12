pub struct Solution;

// This one was a fun one.
// no number meets this condition!
// as the base nears n-2 the representation nears 10 (12, 13, 14, ...)
// and none of these are palindromic
impl Solution {
    #[allow(unused_variables)]
    pub fn is_strictly_palindromic(n: i32) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_strictly_palindromic(9), false);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_strictly_palindromic(4), false);
    }
}
