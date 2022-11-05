/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let str = x.to_string();
        str == str.chars().rev().collect::<String>()
    }
    #[allow(dead_code)]
    pub fn is_palindrome_int(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut copy = x;
        let mut rev = 0;
        while copy > 0 {
            rev = rev * 10 + copy % 10;
            copy /= 10;
        }
        rev == x
    }
}
// @lc code=end

struct Solution;
pub fn main() {
    assert_eq!(Solution::is_palindrome(121), true, "121 is palindrome");
    assert_eq!(
        Solution::is_palindrome(-121),
        false,
        "-121 is not palindrome"
    );
    assert_eq!(Solution::is_palindrome(10), false, "10 is not palindrome");
}
