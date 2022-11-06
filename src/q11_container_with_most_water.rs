/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

// @lc code=start
impl Solution {
    // observation: say L is the left bound, R is the right bound.
    // for all k>L, we can remove h[k] <= h[k+1] because the area will be smaller.
    // for all k<R, we can remove h[k] <= h[k-1] because the area will be smaller.
    // yet we need to move L and R too.
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        while left < right {
            let area = (right - left) as i32 * height[left].min(height[right]);
            max = max.max(area); // largest possible area of width (right - left)
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max
    }
}
// @lc code=end
struct Solution;
pub fn main() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area(vec![1, 1]), 1);
}
