/*
 * @lc app=leetcode id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 */

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        Solution::my_atoi_pablo_plain(s)
    }
    // #[warn(dead_code)]
    // pub fn my_atoi_pablo_regex(s: String) -> i32 {
    //     // regex solution, leet code doesn't support regex
    //     use regex::Regex;
    //     let re = Regex::new(r"^ *([+-]?\d+).*").unwrap();
    //     let num;
    //     if let Some(caps) = re.captures(&s) {
    //         num = caps.get(1).unwrap().as_str().parse::<i32>().unwrap_or(0);
    //     } else {
    //         return 0;
    //     }
    //     if num > i32::max_value() {
    //         return i32::max_value();
    //     }
    //     if num < i32::min_value() {
    //         return i32::min_value();
    //     }
    //     num
    // }
    pub fn my_atoi_pablo_plain(s: String) -> i32 {
        let mut sign = 1;
        let mut result = 0;
        let mut reading_digit = false; // which digit is being read

        for c in s.chars() {
            if !reading_digit {
                match c {
                    '-' => {
                        sign *= -1;
                        reading_digit = true
                    }
                    ' ' => continue,
                    '+' => {
                        reading_digit = true;
                        continue;
                    }
                    '0'..='9' => {
                        reading_digit = true;
                        result = c.to_digit(10).unwrap() as i32;
                    }
                    _ => break,
                }
            } else {
                match c {
                    '0'..='9' => {
                        let digit = c.to_digit(10).unwrap() as i32;
                        if result > (i32::MAX - digit) / 10 {
                            return if sign == 1 { i32::MAX } else { i32::MIN };
                        }
                        result = result * 10 + digit;
                    }
                    _ => break,
                }
            }
        }
        return sign * result;
    }
}

// @lc code=end
struct Solution;
pub fn main() {
    self::_test();
}
#[test]
fn test() {
    self::_test()
}
pub fn _test() {
    assert_eq!(Solution::my_atoi("42".to_string()), 42, "42 is 42");
    assert_eq!(Solution::my_atoi("   -42".to_string()), -42, "-42 is -42");
    assert_eq!(
        Solution::my_atoi("4193 with words".to_string()),
        4193,
        "4193 is 4193"
    );
    assert_eq!(Solution::my_atoi("+1".to_string()), 1, "+1 is 1");
    assert_eq!(
        Solution::my_atoi("+-12".to_string()),
        0,
        "+ and - must be followed by digit"
    );
}
