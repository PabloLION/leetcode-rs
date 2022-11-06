/*
 * @lc app=leetcode id=17 lang=rust
 *
 * [17] Letter Combinations of a Phone Number
 */

// @lc code=start
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut ans = Vec::<String>::new();
        let digits = digits.chars().collect::<Vec<char>>();
        use std::collections::hash_map::HashMap;

        let phone_keys = HashMap::from([
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ]);

        for digit in digits.iter() {
            let mut new_ans = Vec::new();
            let letters = phone_keys.get(digit).unwrap();

            if ans.is_empty() {
                for letter in letters {
                    new_ans.push(letter.to_string());
                }
            } else {
                for letter in letters {
                    for s in ans.iter() {
                        let mut new_s = s.clone();
                        new_s.push(*letter);
                        new_ans.push(new_s);
                    }
                }
            }
            ans = new_ans;
        }
        ans
    }
}
// @lc code=end
struct Solution;
pub fn main() {
    assert_eq!(
        Solution::letter_combinations("23".to_string()).sort(),
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].sort()
    );
    assert_eq!(
        Solution::letter_combinations("".to_string()),
        Vec::<String>::new()
    );
    assert_eq!(
        Solution::letter_combinations("2".to_string()),
        vec!["a", "b", "c"]
    );
}
