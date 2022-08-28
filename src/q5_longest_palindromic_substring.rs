/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut longest_palindrome = String::new();
        let s_chars = s.chars().collect::<Vec<char>>();
        for center in 0..s_chars.len() {
            // for odd-length palindrome
            let mut j = center;
            let mut k = center;
            while j + 1 < s_chars.len() && k > 0 && s_chars[j + 1] == s_chars[k - 1] {
                j += 1;
                k -= 1;
            }
            if j - k + 1 > longest_palindrome.len() {
                longest_palindrome = s[k..=j].to_string();
            }

            // for even-length palindrome
            let mut j = center;
            let mut k = center + 1;
            while j + 1 < s_chars.len() && k >= 1 && s_chars[j + 1] == s_chars[k - 1] {
                j += 1;
                k -= 1;
            }
            if k < j && j - k + 1 > longest_palindrome.len() {
                longest_palindrome = s[k..=j].to_string();
            }
        }
        println!("{}", longest_palindrome);
        longest_palindrome
    }
}
// @lc code=end

struct Solution;
pub fn main() {
    assert!(vec!("bab".to_string(), "aba".to_string())
        .contains(&Solution::longest_palindrome("babad".to_string())));
    assert_eq!(
        Solution::longest_palindrome("cbbd".to_string()),
        "bb".to_string()
    );
    assert_eq!(
        Solution::longest_palindrome("a".to_string()),
        "a".to_string()
    ); // 79/140
    assert_eq!(
        Solution::longest_palindrome("bb".to_string()),
        "bb".to_string()
    ); // 80/140
}
