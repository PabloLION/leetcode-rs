/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let mut max_len = 0;
        let mut start = 0;
        let mut end;
        let mut map = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            if map.contains_key(&c) {
                start = std::cmp::max(start, map[&c] + 1);
            }
            map.insert(c, i);
            end = i;
            max_len = std::cmp::max(max_len, end - start + 1);
        }
        max_len as i32
    }
}
// @lc code=end

struct Solution;
pub fn main() {
    assert_eq!(
        Solution::length_of_longest_substring("abcabcbb".to_string()),
        3
    );
    assert_eq!(
        Solution::length_of_longest_substring("bbbbb".to_string()),
        1
    );
    assert_eq!(
        Solution::length_of_longest_substring("pwwkew".to_string()),
        3
    );
}
