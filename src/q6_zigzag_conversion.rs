/*
 * @lc app=leetcode id=6 lang=rust
 *
 * [6] Zigzag Conversion
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        Solution::convert_forum(s, num_rows)
    }
    pub fn convert_forum(s: String, num_rows: i32) -> String {
        let mut zigzags: Vec<_> = (0..num_rows)
            .chain((1..num_rows - 1).rev())
            .cycle()
            .zip(s.chars())
            .collect();
        zigzags.sort_by_key(|&(row, _)| row);
        zigzags.into_iter().map(|(_, c)| c).collect()
    }
    #[allow(dead_code)]
    pub fn convert_copilot(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let num_rows = num_rows as usize;
        let mut rows = vec![String::new(); num_rows as usize];
        let mut row = 0;
        let mut going_down = false;
        for c in s.chars() {
            rows[row].push(c);
            if row == 0 || row == num_rows - 1 {
                going_down = !going_down;
            }
            if going_down {
                row += 1
            } else {
                row -= 1
            };
        }
        rows.join("")
    }
}
// @lc code=end
struct Solution;
pub fn main() {
    assert_eq!(
        Solution::convert(String::from("LEETCODEISHIRING"), 3),
        String::from("LCIRETOESIIGEDHN")
    );
}
