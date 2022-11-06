/*
 * @lc app=leetcode id=12 lang=rust
 *
 * [12] Integer to Roman
 */

// @lc code=start
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let rom = "IVXLCDM";
        let val = [1, 5, 10, 50, 100, 500, 1000];
        let mut num = num;
        let mut ans = String::new();
        for i in (0..7).rev() {
            while num >= val[i] {
                ans.push(rom.chars().nth(i).unwrap());
                num -= val[i];
            }
        }
        ans = ans.replace("DCCCC", "CM");
        ans = ans.replace("CCCC", "CD");
        ans = ans.replace("LXXXX", "XC");
        ans = ans.replace("XXXX", "XL");
        ans = ans.replace("VIIII", "IX");
        ans = ans.replace("IIII", "IV");
        ans
    }
    #[allow(dead_code)]
    pub fn int_to_roman_copilot(num: i32) -> String {
        let mut ans = String::new();
        let mut num = num;
        for &(v, s) in &[
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ] {
            while num >= v {
                ans.push_str(s);
                num -= v;
            }
        }
        ans
    }
}

// @lc code=end
struct Solution;
pub fn main() {
    assert_eq!(Solution::int_to_roman(3), "III");
    assert_eq!(Solution::int_to_roman(58), "LVIII");
    assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
}
