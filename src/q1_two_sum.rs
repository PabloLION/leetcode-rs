/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(j) = map.get(&complement) {
                return vec![*j as i32, i as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }
}
// @lc code=end

struct Solution {}

pub fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}
