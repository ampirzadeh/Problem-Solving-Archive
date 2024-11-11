use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            let complement = target - nums[i];
            if let Some(&idx) = hm.get(&complement) {
                return vec![idx, i as i32];
            }
            hm.insert(nums[i], i as i32);
        }
        // fallback case
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
