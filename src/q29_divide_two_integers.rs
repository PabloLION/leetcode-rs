/*
 * @lc app=leetcode id=29 lang=rust
 *
 * [29] Divide Two Integers
 */

// @lc code=start
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        dividend.saturating_div(divisor)
    }
}
// @lc code=end

struct Solution;
pub fn main() {
    assert_eq!(Solution::divide(10, 3), 3);
    assert_eq!(Solution::divide(7, -3), -2);
    assert_eq!(Solution::divide(-2147483648, -1), 2147483647);
}
