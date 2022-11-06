/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let s = s
            .replace("CM", "DCCCC")
            .replace("CD", "CCCC")
            .replace("XC", "LXXXX")
            .replace("XL", "XXXX")
            .replace("IX", "VIIII")
            .replace("IV", "IIII");
        for c in s.chars() {
            result += match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }
        result
    }
}
// @lc code=end
struct Solution;
pub fn main() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}
