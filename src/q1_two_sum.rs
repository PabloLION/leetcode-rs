/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    fn two_sum_copilot(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let complement = target - *num;
            if let Some(j) = map.get(&complement) {
                return vec![*j as i32, i as i32];
            }
            map.insert(num, i);

            // another equivalent way to write the lines above:
            // match map.get(&(target - *num)) {
            //     Some(&i2) => return vec![i as i32, i2],
            //     None => map.insert(*num, i as i32),
            // };
        }
        vec![]
    }

    #[allow(dead_code)]
    fn two_sum_pablo(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }

    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Solution::two_sum_copilot(nums, target)
    }
}
// @lc code=end

struct Solution;
pub fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}
