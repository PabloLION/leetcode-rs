/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ans = String::new();
        if strs.is_empty() {
            return ans;
        }
        let mut i = 0;
        loop {
            let mut c = None;
            for s in &strs {
                if i >= s.len() {
                    return ans;
                }
                if c.is_none() {
                    c = s.chars().nth(i);
                } else if c.unwrap() != s.chars().nth(i).unwrap() {
                    return ans;
                }
            }
            ans.push(c.unwrap());
            i += 1;
        }
    }
}
// @lc code=end
struct Solution;
pub fn main() {
    assert_eq!(
        Solution::longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ]),
        "fl".to_string()
    );
    assert_eq!(
        Solution::longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string()
        ]),
        "".to_string()
    );
}
