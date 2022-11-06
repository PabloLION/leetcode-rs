/*
 * @lc app=leetcode id=10 lang=rust
 *
 * [10] Regular Expression Matching
 */

// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;
        for i in 0..=s.len() {
            for j in 1..=p.len() {
                if p[j - 1] == b'.' {
                    dp[i][j] = i > 0 && dp[i - 1][j - 1];
                } else if p[j - 1] == b'*' {
                    dp[i][j] = dp[i][j - 2]
                        || (i > 0 && dp[i - 1][j] && (s[i - 1] == p[j - 2] || p[j - 2] == b'.'));
                } else {
                    dp[i][j] = i > 0 && dp[i - 1][j - 1] && s[i - 1] == p[j - 1];
                }
            }
        }
        dp[s.len()][p.len()]
    }
}
// @lc code=end
struct Solution;
pub fn main() {
    assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
    assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
    assert_eq!(
        Solution::is_match("aab".to_string(), "c*a*b".to_string()),
        true
    );
    assert_eq!(
        Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()),
        false
    );
}
