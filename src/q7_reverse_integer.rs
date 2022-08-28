/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut result = 0;
        while x != 0 {
            let digit = x % 10;
            x /= 10;
            if result > i32::max_value() / 10 || (result == i32::max_value() / 10 && digit > 7) {
                return 0;
            }
            if result < i32::min_value() / 10 || (result == i32::min_value() / 10 && digit < -8) {
                return 0;
            }
            result = result * 10 + digit;
        }
        result
    }
}
// @lc code=end
struct Solution;
pub fn main() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(1534236469), 0); // 1027/1032
}
