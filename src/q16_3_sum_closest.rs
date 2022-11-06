/*
 * @lc app=leetcode id=16 lang=rust
 *
 * [16] 3Sum Closest
 */

// @lc code=start
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut closest = nums[0] + nums[1] + nums[2];
        for i in 0..nums.len() {
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == target {
                    return target;
                }
                if (sum - target).abs() < (closest - target).abs() {
                    closest = sum;
                }
                if sum < target {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        closest
    }
}
// @lc code=end
struct Solution;
pub fn main() {
    assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
}
